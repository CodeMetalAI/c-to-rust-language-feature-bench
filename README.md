# C-to-Rust LLM Transpiler Benchmark

Benchmark suite and evaluation tool for the paper *"Am I a Compiler Yet? A Benchmark for Feature Support in LLM-Based C-to-Rust Translation"*.
The folder `paper_experimental_results` contains the results of the experiments presented in the paper.

## Overview

This tool evaluates LLM accuracy in C to Rust transpilation. The `c11_language_features_benchmark` folder contains 133 targeted test programs derived systematically from the C11 standard, isolating 46 distinct core language features.

Each test compiles and runs both the original C source and the LLM-generated Rust code, then compares exit codes and stdout to determine if the translation was successful.


## Setup

Set API keys as environment variables:

| Variable | Description |
|----------|-------------|
| `OPENAI_API_KEY` | OpenAI models |
| `ANTHROPIC_API_KEY` | Anthropic models |
| `VLLM_URL`, `VLLM_API_KEY` | VLLM-hosted models |
| `OPENROUTER_API_KEY` | OpenRouter supported models |

## Usage

```bash
# List available tests and models
uv run -m test_runner --list-tests
uv run -m test_runner --list-models

# Run specific tests
uv run -m test_runner --test expressions/comma_op_1
uv run -m test_runner --test expressions/comma_op_1 --test declarations/array_decl_1

# Run all tests
uv run -m test_runner --all-tests

# Specify models (overrides config.json)
uv run -m test_runner --all-tests --model gpt-5.2
uv run -m test_runner --all-tests --model gpt-5.2 --model claude-opus-4.5

# Control concurrency
uv run -m test_runner --all-tests --concurrency 32
```

## Configuration

Edit `test_runner/config.json` to add models or adjust settings:

```json
{
  "model_registry": {
    "my-model": {
      "name": "My Model",
      "model_id": "gpt-5.2",
      "provider": "openai",
      "api_key": "${OPENAI_API_KEY}",
      "temperature": 0.4
    }
  },
  "models_to_run": ["my-model"],
  "stdout_match_threshold": 0.9,
  "samples": 3,
  "concurrency": 32
}
```

| Option | Description |
|--------|-------------|
| `samples` | Number of LLM attempts per test. First sample with `match=true` wins. |
| `concurrency` | Maximum concurrent LLM calls (not tests). |
| `stdout_match_threshold` | Similarity threshold (0.0–1.0) for stdout comparison. |

## Stub Detection 

The stub detector used in this work is proprietary to Code Metal.
However, users can instantiate the `default_stub_detector` function to implement their own Rust stub detector, following the methodology described in the paper.

## Output Structure

Results are written to `benchmark_results/<timestamp>/` with a unique timestamp per run. Each run produces a top-level `summary.json` with aggregate statistics, plus per-model/per-test directories containing the source code, generated Rust, compilation logs, and stdout captures for manual inspection and debugging.

```
benchmark_results/<timestamp>/
├── summary.json                 # Aggregate results across all models
└── <model>/<test>/
    ├── result.json              # Test metrics and sample results
    ├── source.c                 # Original C source
    ├── source.compile.log
    ├── source.stdout            # Expected output
    └── sample_001/
        ├── target.rs            # LLM-generated Rust code
        ├── target.compile.log
        └── target.stdout        # Actual output for comparison
```

### Summary JSON Schema

The `summary.json` file contains aggregate results with the following structure:

| Field | Description |
|-------|-------------|
| `run_id` | Timestamp identifier for the benchmark run |
| `timestamp` | ISO 8601 timestamp of when the run started |
| `duration_seconds` | Total wall-clock time for the benchmark |
| `tests_total` | Number of tests executed |
| `models_tested` | List of model keys that were benchmarked |


#### Results by model

Per-model statistics keyed by model name:

| Field | Description |
|-------|-------------|
| `total_tests` | Number of tests run for this model |
| `source_compiled` | Tests where the original C code compiled |
| `target_compiled` | Tests where the generated Rust code compiled |
| `exit_codes_matched` | Tests where Rust exit code matched C exit code |
| `timed_out` | Tests that exceeded the executable timeout |
| `stub_detected_samples` |Samples flagged as stub code |
| `stub_detected_tests` | Tests with at least one stub detection (not necessarily a fail for pass@n with n>1) |
| `passed` | List of test paths that passed |
| `failed` | List of test paths that failed |
| `category_summary` | Pass rates grouped by C language feature |

#### Results by test

Per-test statistics keyed by test path:

| Field | Description |
|-------|-------------|
| `primary_feature` | The C language feature being tested (e.g., `expr.comma`) |
| `passed_models` | List of models that passed this test |
| `failed_models` | List of models that failed this test |
| `pass_rate` | Fraction of models that passed (0.0–1.0) |

#### Cross-model consensus analysis

| Field | Description |
|-------|-------------|
| `all_passed_tests` | Tests that all models passed |
| `all_failed_tests` | Tests that all models failed |
| `mixed_tests` | Tests with inconsistent results across models |

#### Category Summary

Aggregate pass rates by C language feature category (e.g., `expr.comma`, `decl.array`).
