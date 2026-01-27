"""CLI entry point for the benchmark tool."""

import argparse
import asyncio
import sys
from pathlib import Path

from .models import DEFAULT_CONFIG_PATH, BenchmarkConfig, BenchmarkSummary, load_config
from .runner import discover_tests, run_full_benchmark


def parse_args() -> argparse.Namespace:
    """Parse command line arguments."""
    parser = argparse.ArgumentParser(
        description="LLM Transpiler Benchmark - evaluate C++ to Rust transpilation accuracy",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=f"""
Examples:
  # Run with default config (edit {DEFAULT_CONFIG_PATH})
  python -m test_runner --all-tests

  # Run single test with default config
  python -m test_runner --test expressions/comma_op_1

  # Use a custom config file
  python -m test_runner --all-tests --config my_config.json
        """,
    )

    # Test selection
    test_group = parser.add_mutually_exclusive_group()
    test_group.add_argument("--test", type=str, action="append", help="Test path (can specify multiple)")
    test_group.add_argument("--all-tests", action="store_true", help="Run all discovered tests")

    # Configuration
    parser.add_argument("--config", type=Path, help=f"Path to JSON config file (default: {DEFAULT_CONFIG_PATH})")
    parser.add_argument(
        "--model", type=str, action="append", help="Model to run (can specify multiple, overrides config)"
    )
    parser.add_argument("--concurrency", type=int, help="Max concurrent tests (overrides config)")

    # Utility commands
    parser.add_argument("--list-tests", action="store_true", help="List all discovered tests and exit")
    parser.add_argument("--list-models", action="store_true", help="List available models from config and exit")

    return parser.parse_args()


def handle_list_tests() -> int:
    """Handle --list-tests command."""
    tests = discover_tests()
    print(f"Discovered {len(tests)} tests:")
    for test in tests:
        print(f"  {test}")
    return 0


def handle_list_models(config: BenchmarkConfig) -> int:
    """Handle --list-models command."""
    print("Available models (from config):")
    for name, model in config.model_registry.items():
        marker = "*" if name in config.models_to_run else " "
        print(f"  {marker} {name}: {model.name} ({model.provider})")
    print("\n* = included in models_to_run")
    return 0


def resolve_tests(args: argparse.Namespace) -> list[str] | None:
    """Resolve which tests to run from CLI args."""
    if args.test:
        available = set(discover_tests())
        invalid = [t for t in args.test if t not in available]
        if invalid:
            print(f"Error: Unknown test(s): {', '.join(invalid)}", file=sys.stderr)
            print("Use --list-tests to see available tests.", file=sys.stderr)
            return None
        return list(args.test)
    if args.all_tests:
        tests = discover_tests()
        if not tests:
            print("No tests discovered in datasets/language_features/", file=sys.stderr)
            return None
        return tests
    print("Error: Must specify --test or --all-tests", file=sys.stderr)
    return None


def print_summary(summary: BenchmarkSummary, config: BenchmarkConfig) -> None:
    """Print benchmark summary to stdout."""
    print()
    print("=" * 60)
    print("BENCHMARK SUMMARY")
    print("=" * 60)
    print(f"Run ID: {summary.run_id}")
    print(f"Duration: {summary.duration_seconds:.2f}s")
    print(f"Total tests: {summary.tests_total}")
    print()

    for model_name, model_summary in summary.results_by_model.items():
        print(f"Model: {model_name} ({model_summary.duration_seconds:.2f}s)")
        print(f"  Source compiled: {model_summary.source_compiled}/{model_summary.total_tests}")
        print(f"  Target compiled: {model_summary.target_compiled}/{model_summary.total_tests}")
        print(f"  Timed out: {model_summary.timed_out}/{model_summary.total_tests}")
        print(f"  Exit codes matched: {model_summary.exit_codes_matched}/{model_summary.total_tests}")
        total_samples = model_summary.total_tests * summary.samples
        stub_pct = 100 * model_summary.stub_detected_samples / total_samples if total_samples > 0 else 0
        print(
            f"  Stubs detected: {model_summary.stub_detected_samples} samples ({stub_pct:.1f}%), {model_summary.stub_detected_tests} tests"
        )
        pct = 100 * len(model_summary.passed) / model_summary.total_tests if model_summary.total_tests else 0
        print(f"  Passed: {len(model_summary.passed)}/{model_summary.total_tests} ({pct:.0f}%)")
        print()

    print(f"Results written to: {config.output_dir / summary.run_id}")


def main() -> int:
    """Main entry point."""
    args = parse_args()

    # Handle --list-tests (no config needed)
    if args.list_tests:
        return handle_list_tests()

    # Load configuration
    config = load_config(args.config)

    # Handle --list-models
    if args.list_models:
        return handle_list_models(config)

    # Apply CLI overrides
    overrides: dict[str, object] = {}
    if args.model:
        for m in args.model:
            if m not in config.model_registry:
                print(f"Error: Model '{m}' not in registry", file=sys.stderr)
                return 1
        overrides["models_to_run"] = args.model
    if args.concurrency:
        overrides["concurrency"] = args.concurrency
    if overrides:
        config = config.model_copy(update=overrides)

    # Resolve tests
    tests = resolve_tests(args)
    if tests is None:
        return 1

    print("Running benchmark:")
    print(f"  Config: {args.config or DEFAULT_CONFIG_PATH}")
    print(f"  Tests: {len(tests)}")
    print(f"  Models: {', '.join(config.models_to_run)}")
    print(f"  Samples: {config.samples}")
    print(f"  Concurrency: {config.concurrency}")

    # Run benchmark
    summary = asyncio.run(run_full_benchmark(tests=tests, config=config))
    print_summary(summary, config)
    return 0


if __name__ == "__main__":
    sys.exit(main())
