#!/usr/bin/env python3
"""Compute Table 5 (tab:extended_overview) values from the rerun data.

For each test: per-test rate = (samples satisfying predicate) / (total samples).
Per-(model, predicate) number = mean of per-test rates.

Predicates:
    SR -- samples[i]["match"]               (overall success)
    CR -- samples[i]["target_compiled"]     (Rust compiles under safe-Rust)
    SD -- samples[i]["stub_detected"]       (output is a stub)

Inputs (paths relative to this script, in ../):
    ../2026-05-06T18-03-03/  -- temperature 0.2, 5 samples per test
    ../2026-05-10T00-24-35/  -- temperature 0.8, 5 samples per test

Usage:

    python3 paper_experimental_results/scripts/generate_table5.py

    # Save the LaTeX rows for diffing against the table:
    SCRIPT=paper_experimental_results/scripts/generate_table5.py
    python3 $SCRIPT > table5_rows.tex

Output: LaTeX table rows printed to stdout, ordered by vendor tier,
ready to paste into the body of Table 5 (tab:extended_overview).
"""
import json
from pathlib import Path

BASE = Path(__file__).resolve().parent.parent
RUN_DIRS = [
    ("0.2", BASE / "2026-05-06T18-03-03"),
    ("0.8", BASE / "2026-05-10T00-24-35"),
]

# Same vendor-tier groupings as Table 2.
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

FIELDS = ("match", "target_compiled", "stub_detected")


def mean_rate(model_dir: Path, field: str) -> float | None:
    """For each test, fraction of samples with samples[i][field] true; mean over tests."""
    per_test = []
    for rf in model_dir.rglob("result.json"):
        data = json.loads(rf.read_text())
        samples = data.get("samples") or []
        if not samples:
            continue
        c = sum(1 for s in samples if s.get(field))
        per_test.append(c / len(samples))
    return sum(per_test) / len(per_test) if per_test else None


def fmt(r: float | None) -> str:
    return rf"{r*100:.1f}\%" if r is not None else "N/A"


def main() -> None:
    for i, (tier_name, models) in enumerate(MODEL_TIERS):
        if i > 0:
            print(r"\addlinespace[2pt]")
        print(rf"\rowcolor{{gray!15}}\multicolumn{{7}}{{l}}{{\textit{{{tier_name}}}}} \\")
        for mid, display in models:
            cells = []
            for _, run_dir in RUN_DIRS:
                md = run_dir / mid
                if not md.exists():
                    cells.extend(["MISSING"] * 3)
                    continue
                for field in FIELDS:
                    cells.append(fmt(mean_rate(md, field)))
            print(f"{display:<22} & " + " & ".join(cells) + r" \\")


if __name__ == "__main__":
    main()
