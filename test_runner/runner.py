"""Core benchmark runner logic."""

from __future__ import annotations

import asyncio
import difflib
import logging
import re
import time
from collections import Counter
from dataclasses import dataclass
from datetime import UTC, datetime
from pathlib import Path
from typing import Callable, NamedTuple

import yaml

from .llm import ChatModel, ProviderError, create_chat_model
from .models import (
    BenchmarkConfig,
    BenchmarkSummary,
    CompileResult,
    CompileRunResult,
    ExecResult,
    FeatureSummary,
    ModelAgreement,
    ModelBenchmarkResult,
    ModelConfig,
    ModelSummary,
    ProcResult,
    SampleResult,
    TestMeta,
    TestPassRate,
    TestResult,
)

logger = logging.getLogger(__name__)

# The stub detector is proprietary for Code Metal
# you can instantiate the funciton default_stub_detector
# to implement your own stub detector for Rust following the
# description in the paper.
@dataclass
class StubDecision:
    """Result of stub detection check."""

    triggered: bool = False
    reasons: list[str] | None = None


def default_stub_detector(
    rust_code: str,
    source_file_path: Path,
    source_compile_args: list[str],
    sample_dir: Path,
) -> StubDecision:
    """Default stub detector that always returns False (no stub detected).

    Users can replace `stub_detector` with their own implementation to enable
    custom stub detection logic.
    """
    return StubDecision(triggered=False, reasons=None)


# User-replaceable stub detection function
stub_detector: Callable[[str, Path, list[str], Path], StubDecision] = default_stub_detector


DATASETS_ROOT = Path("c11_language_features_benchmark")
FEATURE_TO_CAT_MAP_PATH = Path(__file__).parent / "language_features_c.yaml"
SYSTEM_PROMPT_PATH = Path(__file__).parent / "system_prompt.txt"

# Precompiled regexes for extracting Rust code
_RUST_FENCE = re.compile(r"```rust\s*\r?\n(.*?)```", re.DOTALL)
_ANY_FENCE = re.compile(r"```\s*\r?\n(.*?)```", re.DOTALL)
_CXX_STANDARD = re.compile(r"CMAKE_CXX_STANDARD\s+(\d+)")


def _load_master_feature_to_cat(master_path: Path) -> dict[str, str]:
    data = yaml.safe_load(master_path.read_text())
    if not isinstance(data, list):
        raise ValueError(f"Expected list in master feature file: {master_path}")

    feature_to_cat: dict[str, str] = {}
    for item in data:
        if not isinstance(item, dict):
            continue
        fid = item.get("id")
        cat = item.get("category")
        if isinstance(fid, str):
            feature_to_cat[fid] = cat if isinstance(cat, str) else ""

    return feature_to_cat


def features_map_to_cats(
    tests: list[str],
    root: Path = FEATURE_TO_CAT_MAP_PATH,
) -> list[str]:
    """
    Validate that each test's primary feature maps to a category.

    Returns a list of human-readable error strings.
    Empty list means validation passed.
    """
    feature_to_cat = _load_master_feature_to_cat(root)
    errors: list[str] = []

    for test in tests:
        test_dir = DATASETS_ROOT / test
        feature = get_primary_feature(test_dir)

        if feature is None:
            continue

        if feature not in feature_to_cat:
            errors.append(f"{test}: primary feature '{feature}' missing from language_features_c.yaml")
        elif not feature_to_cat[feature]:
            errors.append(f"{test}: primary feature '{feature}' has no category in language_features_c.yaml")

    return errors


_DISCOVER_EXCLUDE_DIRS: frozenset[str] = frozenset({"combined_programs"})


def discover_tests(root: Path = DATASETS_ROOT) -> list[str]:
    """Discover all tests by presence of description.md, skipping combined_programs."""
    if not root.exists():
        return []
    return sorted(
        str(f.parent.relative_to(root))
        for f in root.rglob("description.md")
        if f.is_file() and f.parent != root and not any(part in _DISCOVER_EXCLUDE_DIRS for part in f.parts)
    )


def find_source(test_dir: Path) -> Path | None:
    """Find C/C++ source file in test directory (deterministic).

    Prefers C++ extensions, falls back to .c files.
    """
    for ext in (".cpp", ".C", ".cc", ".cxx", ".c"):
        matches = sorted(test_dir.glob(f"*{ext}"))
        if matches:
            return matches[0]
    return None


def is_c_source(path: Path) -> bool:
    """Check if source file is C (not C++)."""
    return path.suffix == ".c"


def get_cxx_standard(test_dir: Path) -> str:
    """Get C++ standard from CMakeLists.txt, default to c++17."""
    cmake = test_dir / "CMakeLists.txt"
    if not cmake.exists():
        return "-std=c++17"
    if match := _CXX_STANDARD.search(cmake.read_text()):
        return f"-std=c++{match.group(1)}"
    return "-std=c++17"


def get_c_standard(test_dir: Path) -> str:
    """Get C standard from CMakeLists.txt, default to c11."""
    cmake = test_dir / "CMakeLists.txt"
    if not cmake.exists():
        return "-std=c11"
    # Could parse CMAKE_C_STANDARD if needed
    return "-std=c11"


def get_primary_feature(test_dir: Path) -> str | None:
    """Get primary feature from meta.yaml."""
    for meta_file in test_dir.glob("*.meta.yaml"):
        data = yaml.safe_load(meta_file.read_text())
        meta = TestMeta.model_validate(data)
        if meta.features.primary:
            return meta.features.primary[0]
    return None


def extract_rust_code(text: str) -> str | None:
    """Extract Rust code from LLM response."""
    for rx in (_RUST_FENCE, _ANY_FENCE):
        if match := rx.search(text):
            return match.group(1).strip()
    if "fn main" in text:
        return text.strip()
    return None


def compute_similarity(a: str, b: str) -> float:
    """Compute string similarity (0.0 to 1.0)."""
    if a == b:
        return 1.0
    return difflib.SequenceMatcher(None, a, b).ratio()


async def run_proc(*cmd: str, timeout: float | None = None) -> ProcResult:
    """Run subprocess. exit_code is None on timeout."""
    proc = await asyncio.create_subprocess_exec(
        *cmd,
        stdout=asyncio.subprocess.PIPE,
        stderr=asyncio.subprocess.PIPE,
    )
    try:
        async with asyncio.timeout(timeout):
            out_b, err_b = await proc.communicate()
        return ProcResult(
            exit_code=proc.returncode,
            stdout=out_b.decode("utf-8", "replace"),
            stderr=err_b.decode("utf-8", "replace"),
        )
    except TimeoutError:
        proc.kill()
        await proc.wait()
        return ProcResult(exit_code=None, stdout="", stderr="TIMEOUT")


async def compile_source(compiler: str, args: list[str], source: Path, output: Path) -> CompileResult:
    """Compile source file."""
    proc = await run_proc(compiler, *args, "-o", str(output), str(source))
    log = (proc.stdout + proc.stderr).strip()
    return CompileResult(success=(proc.exit_code == 0), log=log)


async def run_exe(path: Path, timeout: float) -> ExecResult:
    """Run executable."""
    proc = await run_proc(str(path), timeout=timeout)
    return ExecResult(exit_code=proc.exit_code, stdout=proc.stdout)


async def compile_and_run(
    compiler: str, args: list[str], source: Path, binary: Path, timeout: float
) -> CompileRunResult:
    """Compile and optionally run. Returns results (caller writes files)."""
    compile_result = await compile_source(compiler, args, source, binary)
    if not compile_result.success:
        return CompileRunResult(compile=compile_result)
    exec_result = await run_exe(binary, timeout)
    return CompileRunResult(compile=compile_result, exec=exec_result)


async def run_single_sample(
    sample_index: int,
    source_code: str,
    source_file_path: Path,
    source_compile_args: list[str],
    src_exit: int | None,
    src_stdout: str,
    sample_dir: Path,
    chat: ChatModel,
    system_prompt: str,
    config: BenchmarkConfig,
    llm_sem: asyncio.Semaphore,
) -> SampleResult:
    """Run a single sample: LLM call, compile, run, compare."""
    start = time.perf_counter()
    sample_dir.mkdir(parents=True, exist_ok=True)

    try:
        # Generate Rust via LLM (semaphore prevents stampeding the LLM)
        async with llm_sem:
            try:
                async with asyncio.timeout(config.llm_timeout):
                    resp = await asyncio.to_thread(
                        chat.invoke,
                        [
                            {"role": "system", "content": system_prompt},
                            {
                                "role": "user",
                                "content": f"Convert this C/C++ code to Rust:\n\n```c\n{source_code}\n```",
                            },
                        ],
                    )
            except TimeoutError:
                return SampleResult(
                    sample_index=sample_index,
                    duration_seconds=time.perf_counter() - start,
                    timeout_error=True,
                    target_compiled=False,
                    target_compile_errors=None,
                    target_exit_code=None,
                    exit_codes_match=False,
                    stdout_similarity=None,
                    stdout_match=False,
                    match=False,
                )
            except ProviderError as e:
                logger.warning(
                    "Provider error in sample %d for %s: %s",
                    sample_index,
                    sample_dir.parent.name,
                    e,
                )
                return SampleResult(
                    sample_index=sample_index,
                    duration_seconds=time.perf_counter() - start,
                    provider_error=True,
                    timeout_error=False,
                    target_compiled=False,
                    target_compile_errors=None,
                    target_exit_code=None,
                    exit_codes_match=False,
                    stdout_similarity=None,
                    stdout_match=False,
                    match=False,
                )

        rust_code = extract_rust_code(str(resp.content)) or "// Failed to extract"

        # Run stub detection (user-replaceable via stub_detector)
        stub_decision = stub_detector(
            rust_code=rust_code,
            source_file_path=source_file_path,
            source_compile_args=source_compile_args,
            sample_dir=sample_dir,
        )
        if stub_decision.triggered:
            logger.warning(
                "Stub detected in sample %d for %s: %s",
                sample_index,
                sample_dir.parent.name,
                "; ".join(stub_decision.reasons or []),
            )

        # Write and compile Rust
        tgt_rs = sample_dir / "target.rs"
        tgt_rs.write_text(rust_code)
        tgt = await compile_and_run(
            "rustc", ["-F", "unsafe_code"], tgt_rs, sample_dir / "target", config.executable_timeout
        )
        (sample_dir / "target.compile.log").write_text(tgt.compile.log)
        if tgt.exec:
            (sample_dir / "target.stdout").write_text(tgt.exec.stdout)

        # Compute results
        tgt_exit = tgt.exec.exit_code if tgt.exec else None
        similarity = (
            compute_similarity(src_stdout, tgt.exec.stdout)
            if tgt.exec and src_exit is not None and tgt_exit is not None
            else None
        )
        exit_match = src_exit == tgt_exit
        stdout_match = similarity is not None and similarity > config.stdout_match_threshold

        return SampleResult(
            sample_index=sample_index,
            duration_seconds=time.perf_counter() - start,
            timeout_error=False,
            target_compiled=tgt.compile.success,
            target_compile_errors=tgt.compile.log if not tgt.compile.success else None,
            target_exit_code=tgt_exit,
            exit_codes_match=exit_match,
            stdout_similarity=similarity,
            stdout_match=stdout_match,
            stub_detected=stub_decision.triggered,
            stub_reasons=stub_decision.reasons if stub_decision.triggered else None,
            match=exit_match and stdout_match and not stub_decision.triggered,
        )
    except asyncio.CancelledError:
        raise
    except Exception as e:
        logger.warning(
            "Sample failed: test=%s sample=%d err=%s repr=%r cause=%r context=%r",
            sample_dir.parent.name,
            sample_index,
            type(e).__name__,
            e,
            getattr(e, "__cause__", None),
            getattr(e, "__context__", None),
        )

        return SampleResult(
            sample_index=sample_index,
            duration_seconds=time.perf_counter() - start,
            timeout_error=False,
            target_compiled=False,
            target_compile_errors=None,
            target_exit_code=None,
            exit_codes_match=False,
            stdout_similarity=None,
            stdout_match=False,
            match=False,
        )


async def _run_samples_until_match(
    *,
    source_code: str,
    source_file: Path,
    source_compile_args: list[str],
    src_exit: int | None,
    src_stdout: str,
    result_dir: Path,
    chat: ChatModel,
    system_prompt: str,
    config: BenchmarkConfig,
    llm_sem: asyncio.Semaphore,
) -> tuple[list[SampleResult], SampleResult | None]:
    """Spawn samples concurrently, replacing provider-errored ones until max_retries is exhausted."""
    max_total = config.samples + config.max_retries
    next_idx = 1
    in_flight: set[asyncio.Task[SampleResult]] = set()

    def _spawn() -> None:
        nonlocal next_idx
        if next_idx <= max_total:
            in_flight.add(
                asyncio.create_task(
                    run_single_sample(
                        sample_index=next_idx,
                        source_code=source_code,
                        source_file_path=source_file,
                        source_compile_args=source_compile_args,
                        src_exit=src_exit,
                        src_stdout=src_stdout,
                        sample_dir=result_dir / f"sample_{next_idx:03d}",
                        chat=chat,
                        system_prompt=system_prompt,
                        config=config,
                        llm_sem=llm_sem,
                    )
                )
            )
            next_idx += 1

    for _ in range(config.samples):
        _spawn()

    samples: list[SampleResult] = []
    winning: SampleResult | None = None

    while in_flight:
        done, _ = await asyncio.wait(in_flight, return_when=asyncio.FIRST_COMPLETED)
        in_flight -= done
        for task in done:
            sample = task.result()
            samples.append(sample)
            if sample.provider_error:
                _spawn()  # no-op once max_total is exhausted
            elif sample.match and not winning:
                winning = sample

    return sorted(samples, key=lambda s: s.sample_index), winning


async def run_single_test(
    test_name: str,
    model_config: ModelConfig,
    output_dir: Path,
    system_prompt: str,
    config: BenchmarkConfig,
    llm_sem: asyncio.Semaphore,
    chat: ChatModel,
) -> TestResult:
    """Run a single benchmark test with multiple samples."""
    start = time.perf_counter()
    timestamp = datetime.now(UTC)
    test_dir = DATASETS_ROOT / test_name
    result_dir = output_dir / test_name
    result_dir.mkdir(parents=True, exist_ok=True)

    primary_feature = get_primary_feature(test_dir)

    source_file = find_source(test_dir)
    if not source_file:
        raise FileNotFoundError(f"No C/C++ source in {test_dir}")
    source_code = source_file.read_text()
    (result_dir / f"source{source_file.suffix}").write_text(source_code)

    if is_c_source(source_file):
        compiler, std_flag = "gcc", get_c_standard(test_dir)
    else:
        compiler, std_flag = "g++", get_cxx_standard(test_dir)
    src = await compile_and_run(compiler, [std_flag], source_file, result_dir / "source", config.executable_timeout)
    (result_dir / "source.compile.log").write_text(src.compile.log)
    if src.exec:
        (result_dir / "source.stdout").write_text(src.exec.stdout)

    if not src.compile.success:
        result = TestResult(
            test_name=test_name,
            llm_name=model_config.name,
            timestamp=timestamp,
            duration_seconds=time.perf_counter() - start,
            timeout_error=False,
            primary_feature=primary_feature,
            source_compiled=False,
            source_compile_errors=src.compile.log,
            source_exit_code=None,
            samples=[],
            winning_sample=None,
            match=False,
        )
        (result_dir / "result.json").write_text(result.model_dump_json(indent=2))
        return result

    src_exit = src.exec.exit_code if src.exec else None
    src_stdout = src.exec.stdout if src.exec else ""

    attempted, winning = await _run_samples_until_match(
        source_code=source_code,
        source_file=source_file,
        source_compile_args=[std_flag],
        src_exit=src_exit,
        src_stdout=src_stdout,
        result_dir=result_dir,
        chat=chat,
        system_prompt=system_prompt,
        config=config,
        llm_sem=llm_sem,
    )
    timed_out = (winning is None) and all(s.timeout_error for s in attempted)

    result = TestResult(
        test_name=test_name,
        llm_name=model_config.name,
        timestamp=timestamp,
        duration_seconds=time.perf_counter() - start,
        timeout_error=timed_out,
        primary_feature=primary_feature,
        source_compiled=True,
        source_compile_errors=None,
        source_exit_code=src_exit,
        samples=attempted,
        winning_sample=winning.sample_index if winning else None,
        match=winning is not None,
    )
    (result_dir / "result.json").write_text(result.model_dump_json(indent=2))
    return result


class FeaturePass(NamedTuple):
    """A feature and whether it passed."""

    feature: str
    passed: bool


def _build_category_summary(items: list[FeaturePass]) -> dict[str, FeatureSummary]:
    """Build category summary from feature/pass pairs (keyed by primary feature)."""
    totals: Counter[str] = Counter()
    passed: Counter[str] = Counter()

    for feature, is_passed in items:
        totals[feature] += 1
        if is_passed:
            passed[feature] += 1

    return {
        feat: FeatureSummary(passed=passed[feat], total=total, pass_rate=passed[feat] / total)
        for feat, total in sorted(totals.items())
    }


async def run_model_benchmark(
    tests: list[str], model_name: str, output_dir: Path, config: BenchmarkConfig, progress: bool = True
) -> ModelBenchmarkResult:
    start = time.perf_counter()
    model_config = config.get_model(model_name)
    prompt = SYSTEM_PROMPT_PATH.read_text()
    chat = create_chat_model(model_config, config.temperature)
    llm_sem = asyncio.Semaphore(config.concurrency)
    test_sem = asyncio.Semaphore(config.concurrency)
    total = len(tests)
    completed = 0

    async def run(test: str) -> TestResult:
        nonlocal completed
        async with test_sem:
            result = await run_single_test(test, model_config, output_dir, prompt, config, llm_sem, chat)
        completed += 1
        if progress:
            any_compiled = any(s.target_compiled for s in result.samples) if result.samples else False
            any_stub = any(s.stub_detected for s in result.samples) if result.samples else False
            if result.match:
                status = f"✓{result.winning_sample}"
            elif any_compiled:
                status = "✗"
            else:
                status = "⚠"
            stub_indicator = " [STUB]" if any_stub else ""
            print(f"  [{completed}/{total}] {status} {test} ({result.duration_seconds:.1f}s){stub_indicator}")
        return result

    raw_results = await asyncio.gather(*(run(t) for t in tests), return_exceptions=True)

    # Separate successful results from exceptions
    results: list[TestResult] = []
    errors: list[tuple[str, BaseException]] = []
    for test, result in zip(tests, raw_results):
        if isinstance(result, BaseException):
            errors.append((test, result))
        else:
            results.append(result)

    # Fail fast: raise first error with context about how many tests failed
    if errors:
        first_test, first_error = errors[0]
        raise RuntimeError(f"{len(errors)} test(s) failed to run. First: {first_test}") from first_error

    passed = [r.test_name for r in results if r.match]
    failed = [r.test_name for r in results if not r.match]

    # Aggregate: count tests where any sample compiled/matched
    target_compiled = sum(1 for r in results if any(s.target_compiled for s in r.samples))
    exit_codes_matched = sum(1 for r in results if any(s.exit_codes_match for s in r.samples))

    # Aggregate stub detection stats
    stub_detected_samples = sum(sum(1 for s in r.samples if s.stub_detected) for r in results)
    stub_detected_tests = sum(1 for r in results if any(s.stub_detected for s in r.samples))

    # Aggregate provider error stats
    provider_error_samples = sum(sum(1 for s in r.samples if s.provider_error) for r in results)
    provider_error_tests = sum(1 for r in results if any(s.provider_error for s in r.samples))

    summary = ModelSummary(
        model_name=model_name,
        total_tests=len(results),
        source_compiled=sum(r.source_compiled for r in results),
        timed_out=sum(r.timeout_error for r in results),
        target_compiled=target_compiled,
        exit_codes_matched=exit_codes_matched,
        stub_detected_samples=stub_detected_samples,
        stub_detected_tests=stub_detected_tests,
        provider_error_samples=provider_error_samples,
        provider_error_tests=provider_error_tests,
        duration_seconds=time.perf_counter() - start,
        passed=passed,
        failed=failed,
        category_summary=_build_category_summary(
            [FeaturePass(r.primary_feature, r.match) for r in results if r.primary_feature]
        ),
    )
    return ModelBenchmarkResult(summary=summary, results=list(results))


def compute_results_by_test(
    tests: list[str],
    results_by_model: dict[str, ModelSummary],
    test_features: dict[str, str | None],
) -> dict[str, TestPassRate]:
    """Compute per-test pass rates across models."""
    model_count = len(results_by_model)
    results: dict[str, TestPassRate] = {}
    for test in tests:
        passed_models = [m for m, s in results_by_model.items() if test in s.passed]
        failed_models = [m for m, s in results_by_model.items() if test in s.failed]
        results[test] = TestPassRate(
            primary_feature=test_features.get(test),
            passed_models=passed_models,
            failed_models=failed_models,
            passed_models_count=len(passed_models),
            model_count=model_count,
            pass_rate=len(passed_models) / model_count if model_count else 0.0,
        )
    return results


def compute_model_agreement(results_by_test: dict[str, TestPassRate]) -> ModelAgreement:
    """Categorize tests by model agreement."""
    all_passed = [t for t, r in results_by_test.items() if r.passed_models_count == r.model_count]
    all_failed = [t for t, r in results_by_test.items() if r.passed_models_count == 0]
    mixed = [t for t, r in results_by_test.items() if 0 < r.passed_models_count < r.model_count]
    return ModelAgreement(all_passed_tests=all_passed, all_failed_tests=all_failed, mixed_tests=mixed)


def compute_category_summary(results_by_test: dict[str, TestPassRate]) -> dict[str, FeatureSummary]:
    """Compute pass rates by C++ feature category (unanimous pass across all models)."""
    return _build_category_summary(
        [
            FeaturePass(r.primary_feature, r.passed_models_count == r.model_count)
            for r in results_by_test.values()
            if r.primary_feature
        ]
    )


async def run_full_benchmark(tests: list[str] | None, config: BenchmarkConfig) -> BenchmarkSummary:
    """Run full benchmark across all configured models."""
    start = time.perf_counter()
    timestamp = datetime.now(UTC)
    run_id = timestamp.strftime("%Y-%m-%dT%H-%M-%S")
    tests = tests or discover_tests()

    errors = features_map_to_cats(tests, FEATURE_TO_CAT_MAP_PATH)
    if errors:
        msg = "\n".join(errors)
        raise RuntimeError(f"Feature maps to category check failed ({len(errors)} error(s)):\n{msg}")

    run_dir = config.output_dir / run_id
    run_dir.mkdir(parents=True, exist_ok=True)
    print(f"Output: {run_dir.absolute()}")

    # Run each model
    results_by_model: dict[str, ModelSummary] = {}
    test_features: dict[str, str | None] = {}
    total = 0
    for i, model in enumerate(config.models_to_run, 1):
        print(f"\n[{i}/{len(config.models_to_run)}] {model}")
        benchmark = await run_model_benchmark(tests, model, run_dir / model, config)
        results_by_model[model] = benchmark.summary
        if not test_features:
            test_features = {r.test_name: r.primary_feature for r in benchmark.results}
        total += benchmark.summary.total_tests
        print(
            f"  → {len(benchmark.summary.passed)}/{benchmark.summary.total_tests} passed ({benchmark.summary.duration_seconds:.1f}s)"
        )

    # Compute aggregates
    results_by_test = compute_results_by_test(tests, results_by_model, test_features)

    result = BenchmarkSummary(
        run_id=run_id,
        timestamp=timestamp,
        duration_seconds=time.perf_counter() - start,
        concurrency=config.concurrency,
        samples=config.samples,
        temperature=config.temperature,
        tests_total=total,
        models_tested=config.models_to_run,
        results_by_model=results_by_model,
        results_by_test=results_by_test,
        model_agreement=compute_model_agreement(results_by_test),
        category_summary=compute_category_summary(results_by_test),
    )
    (run_dir / "summary.json").write_text(result.model_dump_json(indent=2))
    return result
