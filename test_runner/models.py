"""Pydantic DTOs for benchmark configuration and results."""

from __future__ import annotations

from datetime import datetime
from pathlib import Path
from typing import Any, Literal

from pydantic import BaseModel, ConfigDict, Field, model_validator


class ModelConfig(BaseModel):
    """Configuration for a single benchmark model."""

    model_config = ConfigDict(extra="forbid", frozen=True)

    name: str = Field(description="Display name for the model")
    model_id: str = Field(description="Model identifier for the API")
    provider: Literal["openai", "anthropic", "vllm"] = Field(description="LLM provider")
    api_key: str = Field(description="API key, literal or ${ENV_VAR}")
    base_url: str | None = Field(default=None, description="Base URL for API, literal or ${ENV_VAR}")
    temperature: float = Field(description="Sampling temperature")
    meta: dict[str, Any] | None = Field(
        default=None,
        description="Optional model metadata (e.g. domains/tags/size buckets) used for selection/grouping"
    )


class BenchmarkConfig(BaseModel):
    """Configuration for benchmark execution."""

    model_config = ConfigDict(extra="forbid", frozen=True)

    model_registry: dict[str, ModelConfig] = Field(description="Available models keyed by short name")
    models_to_run: list[str] = Field(description="Model keys to benchmark")
    stdout_match_threshold: float = Field(ge=0.0, le=1.0, description="Similarity threshold for stdout match")
    concurrency: int = Field(ge=1, description="Max concurrent LLM calls")
    samples: int = Field(ge=1, description="Number of samples per test; first match wins")
    output_dir: Path = Field(description="Output directory for results")
    executable_timeout: float = Field(ge=1.0, description="Timeout in seconds for running executables")
    llm_timeout: float = Field(ge=10.0, description="Timeout in seconds for LLM API calls")

    @model_validator(mode="after")
    def validate_models_to_run(self) -> BenchmarkConfig:
        """Ensure all models_to_run exist in registry."""
        for name in self.models_to_run:
            if name not in self.model_registry:
                raise ValueError(f"Model '{name}' not found in model_registry")
        return self

    def get_model(self, name: str) -> ModelConfig:
        """Get a model config by key."""
        return self.model_registry[name]


DEFAULT_CONFIG_PATH = Path(__file__).parent / "config.json"


def load_config(path: Path | None = None) -> BenchmarkConfig:
    """Load benchmark configuration from JSON file."""
    config_path = path or DEFAULT_CONFIG_PATH
    if not config_path.exists():
        raise FileNotFoundError(f"Config file not found: {config_path}")
    return BenchmarkConfig.model_validate_json(config_path.read_text())


class ProcResult(BaseModel):
    """Result of running a subprocess."""

    exit_code: int | None  # None if timed out
    stdout: str
    stderr: str


class CompileResult(BaseModel):
    """Result of compiling source code."""

    success: bool
    log: str


class ExecResult(BaseModel):
    """Result of running an executable."""

    exit_code: int | None  # None if timed out
    stdout: str


class CompileRunResult(BaseModel):
    """Result of compiling and running source code."""

    compile: CompileResult
    exec: ExecResult | None = None  # None if compile failed


class SampleResult(BaseModel):
    """Result of a single sample attempt."""

    model_config = ConfigDict(extra="forbid", frozen=True)

    sample_index: int = Field(description="1-based sample index")
    duration_seconds: float = Field(description="Time taken for this sample")
    timeout_error: bool = Field(default=False, description="Attempt exceeded allotted time")
    target_compiled: bool = Field(description="Whether Rust code compiled")
    target_compile_errors: str | None = Field(default=None, description="Rust compile errors if any")
    target_exit_code: int | None = Field(default=None, description="Rust executable exit code")
    exit_codes_match: bool = Field(description="Whether source and target exit codes match")
    stdout_similarity: float | None = Field(default=None, description="Similarity ratio 0.0-1.0")
    stdout_match: bool = Field(description="Whether stdout similarity exceeds threshold")
    match: bool = Field(description="True if exit_codes_match and stdout_match")
    stub_detected: bool = Field(default=False, description="Whether stub patterns were detected")
    stub_reasons: list[str] | None = Field(default=None, description="Reasons for stub detection")


class TestFeatures(BaseModel):
    """Features exercised by a test."""

    primary: list[str] = Field(default_factory=list, description="Primary features tested")
    incidental: list[str] = Field(default_factory=list, description="Incidental features used")


class SourceInfo(BaseModel):
    """Source information for a test."""

    section: list[str] = Field(default_factory=list, description="Section from C11 standard the test is derived from")
    clause:  list[int] = Field(default_factory=list, description="Clause from C11 standard the test is derived from")
    example: list[int] = Field(default_factory=list, description="Example from C11 standard the test is derived from")


class TestMeta(BaseModel):
    """Metadata from a test's .meta.yaml file."""

    file: str = Field(description="Source file name")
    author: str = Field(description="Author of the test (e.g., 'gcc', 'code metal')")
    std_source: SourceInfo = Field(default_factory=SourceInfo, description="Origin of the test")
    min_std: str | None = Field(default=None, description="Minimum C/C++ standard required")
    features: TestFeatures = Field(default_factory=TestFeatures, description="C/C++ features exercised")


class TestResult(BaseModel):
    """Result of a test with multiple samples."""

    model_config = ConfigDict(extra="forbid", frozen=True)

    test_name: str = Field(description="Test path relative to datasets/language_features")
    llm_name: str = Field(description="Display name of the LLM used")
    timestamp: datetime = Field(description="When the test started")
    duration_seconds: float = Field(description="Total time for all samples")
    timeout_error: bool = Field(default=False, description="Attempt exceeded allotted time")
    primary_feature: str | None = Field(default=None, description="Primary C++ feature tested (e.g., 'expr.comma')")

    source_compiled: bool = Field(description="Whether C/C++ source compiled")
    source_compile_errors: str | None = Field(default=None, description="C/C++ compile errors if any")
    source_exit_code: int | None = Field(default=None, description="C/C++ executable exit code")

    samples: list[SampleResult] = Field(description="Results for each sample attempt")
    winning_sample: int | None = Field(default=None, description="Index of first matching sample, None if none matched")
    match: bool = Field(description="True if any sample matched")


class ModelSummary(BaseModel):
    """Aggregate results for a single model."""

    model_config = ConfigDict(extra="forbid", frozen=True)

    model_name: str = Field(description="Model key from config")
    temperature: float = Field(description="Temperature for all tests run")
    total_tests: int = Field(description="Number of tests run")
    source_compiled: int = Field(description="Tests where C/C++ compiled")
    timed_out: int = Field(description="Tests that took longer than allocated timed")
    target_compiled: int = Field(description="Tests where any sample's Rust compiled")
    exit_codes_matched: int = Field(description="Tests where any sample's exit code matched")
    stub_detected_samples: int = Field(description="Total number of samples where stubs were detected")
    stub_detected_tests: int = Field(description="Number of tests where at least one sample had stubs detected")
    duration_seconds: float = Field(description="Total time for this model")
    passed: list[str] = Field(description="Test names that passed")
    failed: list[str] = Field(description="Test names that failed")
    category_summary: dict[str, FeatureSummary] = Field(description="Pass rates by primary feature for this model")


class ModelBenchmarkResult(BaseModel):
    """Result of running benchmark for a single model."""

    summary: ModelSummary = Field(description="Aggregate summary for the model")
    results: list[TestResult] = Field(description="Individual test results")


class TestPassRate(BaseModel):
    """Per-test pass rate across models."""

    model_config = ConfigDict(extra="forbid", frozen=True)

    primary_feature: str | None = Field(default=None, description="Primary C feature tested")
    passed_models: list[str] = Field(description="Model keys that passed this test")
    failed_models: list[str] = Field(description="Model keys that failed this test")
    passed_models_count: int = Field(description="Number of models that passed")
    model_count: int = Field(description="Total models evaluated")
    pass_rate: float = Field(description="Fraction of models that passed (0.0-1.0)")


class ModelAgreement(BaseModel):
    """Consensus analysis across models."""

    model_config = ConfigDict(extra="forbid", frozen=True)

    all_passed_tests: list[str] = Field(description="Tests where all models passed")
    all_failed_tests: list[str] = Field(description="Tests where no model passed")
    mixed_tests: list[str] = Field(description="Tests with mixed pass/fail results")


class FeatureSummary(BaseModel):
    """Pass rate for a specific C++ feature."""

    model_config = ConfigDict(extra="forbid", frozen=True)

    passed: int = Field(description="Number of tests passed")
    total: int = Field(description="Total number of tests")
    pass_rate: float = Field(description="Fraction of tests passed (0.0-1.0)")


class BenchmarkSummary(BaseModel):
    """Summary of an entire benchmark run."""

    model_config = ConfigDict(extra="forbid", frozen=True)

    run_id: str = Field(description="Timestamp-based run identifier")
    timestamp: datetime = Field(description="When the benchmark started")
    duration_seconds: float = Field(description="Total benchmark duration")
    concurrency: int = Field(description="Max concurrent LLM calls")
    samples: int = Field(description="Samples per test setting used")
    tests_total: int = Field(description="Total test executions across all models")
    models_tested: list[str] = Field(description="Model keys that were tested")
    results_by_model: dict[str, ModelSummary] = Field(description="Per-model summary results")
    results_by_test: dict[str, TestPassRate] = Field(description="Per-test pass rates")
    model_agreement: ModelAgreement = Field(description="Consensus analysis across models")
    category_summary: dict[str, FeatureSummary] = Field(
        description="Pass rates by primary feature (unanimous across all models)"
    )
