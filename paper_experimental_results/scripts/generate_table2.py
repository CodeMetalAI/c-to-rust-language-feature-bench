#!/usr/bin/env python3
"""Compute Table II mean success-rate values from the rerun data.

For each test: per-test mean SR = (correct samples) / (total samples).
Per-model number = mean of per-test mean SRs.

Inputs (paths relative to this script, in ../):
    ../2026-05-06T18-03-03/  -- temperature 0.2, 5 samples per test
    ../2026-05-10T00-24-35/  -- temperature 0.8, 5 samples per test

Each input directory must contain one subdirectory per model, each
populated with per-test ``result.json`` files (the runner's default
output format with all 5 samples retained).

Usage:

    python3 paper_experimental_results/scripts/generate_table2.py

    # Save the LaTeX rows for diffing against the table:
    python3 paper_experimental_results/scripts/generate_table2.py \
        > table2_rows.tex

Output: LaTeX table rows printed to stdout, ordered by model tier,
ready to paste into Table II (replacing the ``XX.X%`` placeholder
cells). The script reads from disk only -- nothing is written by
default.
"""
import json
from pathlib import Path

BASE = Path(__file__).resolve().parent.parent
RUN_DIRS = [
    ("0.2", BASE / "2026-05-06T18-03-03"),
    ("0.8", BASE / "2026-05-10T00-24-35"),
]

# Models grouped by vendor tier (matches original Table II organization).
# Each entry: (results_dir_id, display_name).
MODEL_TIERS = [
    ("Small Models", [
        ("gemma-3-12b",            "Gemma 3 12B Instruct"),
        ("llama-3.1-8b-instruct",  "Llama 3.1 8B Instruct"),
    ]),
    ("Large Models", [
        ("llama-3.1-70b",          "Llama 3.1 70B"),
        ("deepseek-v3.2",          "DeepSeek V3.2"),
    ]),
    ("Programming-Specialized Models", [
        ("grok-code-fast-1",       "Grok Code Fast 1"),
        ("mistral-codestral-2508", "Mistral Codestral 2508"),
        ("gpt-5.2-codex",          "GPT-5.2-codex"),
    ]),
    ("Flagship Models", [
        ("gpt-4o",                 "GPT-4o"),
        ("gpt-4-turbo",            "GPT-4o turbo"),
        ("claude-opus-4.5",        "Claude Opus 4.5"),
        ("claude-sonnet-4.5",      "Claude Sonnet 4.5"),
    ]),
    ("Lightweight Flagship Models", [
        ("gpt-4o-mini",            "GPT-4o mini"),
    ]),
]


def mean_sr(model_dir: Path) -> float | None:
    per_test_rates = []
    for rf in model_dir.rglob("result.json"):
        data = json.loads(rf.read_text())
        samples = data.get("samples") or []
        if not samples:
            continue
        c = sum(1 for s in samples if s.get("match"))
        per_test_rates.append(c / len(samples))
    return sum(per_test_rates) / len(per_test_rates) if per_test_rates else None


def main() -> None:
    for i, (tier_name, models) in enumerate(MODEL_TIERS):
        if i > 0:
            print(r"\addlinespace[2pt]")
        print(rf"\rowcolor{{gray!15}}\multicolumn{{3}}{{l}}{{\textit{{{tier_name}}}}} \\")
        for mid, display in models:
            cells = []
            for _, run_dir in RUN_DIRS:
                md = run_dir / mid
                if not md.exists():
                    cells.append("MISSING")
                    continue
                sr = mean_sr(md)
                cells.append(rf"{sr*100:.1f}\%" if sr is not None else "N/A")
            print(f"{display:<22} & " + " & ".join(cells) + r" \\")


if __name__ == "__main__":
    main()
