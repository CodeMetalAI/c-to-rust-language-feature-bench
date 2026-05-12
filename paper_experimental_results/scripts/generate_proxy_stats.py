#!/usr/bin/env python3
"""Compute the combined-vs-individual proxy statistics for the new section.

For each model, per-test mean SR = matching_samples / total_samples (5 samples).
We compare per-test mean SR on three individual tests with the per-test mean SR
on the two combined programs that subsume them.

Tests:
  expressions/array_subscripting
  expressions/struct_and_union_1
  declarations/type_def_2
  combined_programs/combined_features
  combined_programs/pathfinder

Data: ../2026-05-11T22-01-24/ (temp=0.2, 5 samples/test)

Usage:
    python3 paper_experimental_results/scripts/generate_proxy_stats.py

Prints:
  - a markdown summary table (12 rows x 5 cols),
  - aggregate proxy statistics (forward, converse, mean-vs-mean),
  - LaTeX-ready rows for tab:combined-proxy.
"""
import json
from pathlib import Path

BASE = Path(__file__).resolve().parent.parent / "2026-05-11T22-01-24"

TESTS = [
    ("array_sub",     "expressions/array_subscripting"),
    ("struct_member", "expressions/struct_and_union_1"),
    ("type_def",      "declarations/type_def_2"),
    ("combined",      "combined_programs/combined_features"),
    ("pathfinder",    "combined_programs/pathfinder"),
]

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


def per_test_sr(model_dir: Path, test_rel: str) -> float | None:
    rf = model_dir / test_rel / "result.json"
    if not rf.exists():
        return None
    data = json.loads(rf.read_text())
    samples = data.get("samples") or []
    if not samples:
        return None
    c = sum(1 for s in samples if s.get("match"))
    return c / len(samples)


def main() -> None:
    rows = []
    for tier_name, models in MODEL_TIERS:
        for mid, display in models:
            md = BASE / mid
            srs = {key: per_test_sr(md, rel) for key, rel in TESTS}
            rows.append((tier_name, display, srs))

    # Markdown table
    print("# Per-test mean SR (temp=0.2, 5 samples)\n")
    header = ["Model", "array_sub", "struct_member", "type_def", "combined", "pathfinder"]
    print("| " + " | ".join(header) + " |")
    print("|" + "|".join("---" for _ in header) + "|")
    for tier, display, srs in rows:
        cells = [display]
        for key, _ in TESTS:
            v = srs[key]
            cells.append("N/A" if v is None else f"{v*100:.0f}%")
        print("| " + " | ".join(cells) + " |")

    # Aggregate stats
    print("\n# Aggregate statistics\n")

    valid = [(d, s) for (_, d, s) in rows if all(v is not None for v in s.values())]
    print(f"Models with full data: {len(valid)} / {len(rows)}")

    # Forward direction: model-test pairs where individual=0 and combined=0
    pairs = []
    for d, s in valid:
        for key in ("array_sub", "struct_member", "type_def"):
            pairs.append((d, key, s[key], s["combined"]))
    indiv_fail = [(d, k, i, c) for (d, k, i, c) in pairs if i == 0.0]
    indiv_fail_and_combined_fail = [(d, k, i, c) for (d, k, i, c) in indiv_fail if c == 0.0]
    print(f"\nForward (individual=0 -> combined=0):")
    print(f"  {len(indiv_fail_and_combined_fail)} / {len(indiv_fail)} (model, individual-test) pairs")

    # Stronger forward: any individual at 0 -> combined at 0
    any_indiv_zero = [(d, s) for (d, s) in valid
                      if min(s["array_sub"], s["struct_member"], s["type_def"]) == 0.0]
    any_zero_combined_zero = [(d, s) for (d, s) in any_indiv_zero if s["combined"] == 0.0]
    print(f"\nForward (any individual=0 for model -> combined=0):")
    print(f"  {len(any_zero_combined_zero)} / {len(any_indiv_zero)} models")

    # Combined SR vs min individual SR
    le_min = [(d, s) for (d, s) in valid
              if s["combined"] <= min(s["array_sub"], s["struct_member"], s["type_def"]) + 1e-9]
    print(f"\nCombined SR <= min(individual SRs): {len(le_min)} / {len(valid)} models")

    # Combined SR vs mean individual SR
    le_mean = [(d, s) for (d, s) in valid
               if s["combined"] <= (s["array_sub"] + s["struct_member"] + s["type_def"]) / 3 + 1e-9]
    print(f"Combined SR <= mean(individual SRs): {len(le_mean)} / {len(valid)} models")

    # Converse: all individuals = 1.0 -> combined = 1.0
    all_indiv_one = [(d, s) for (d, s) in valid
                     if min(s["array_sub"], s["struct_member"], s["type_def"]) == 1.0]
    all_one_combined_one = [(d, s) for (d, s) in all_indiv_one if s["combined"] == 1.0]
    print(f"\nConverse (all individuals=1.0 -> combined=1.0):")
    print(f"  {len(all_one_combined_one)} / {len(all_indiv_one)} models")

    # Aggregate means
    n = len(valid)
    if n:
        mean_indiv = sum((s["array_sub"] + s["struct_member"] + s["type_def"]) / 3
                         for _, s in valid) / n
        mean_combined = sum(s["combined"] for _, s in valid) / n
        mean_pathfinder = sum(s["pathfinder"] for _, s in valid) / n
        print(f"\nMean over {n} models:")
        print(f"  mean(individual mean SR)  = {mean_indiv*100:.1f}%")
        print(f"  mean(combined SR)         = {mean_combined*100:.1f}%")
        print(f"  mean(pathfinder SR)       = {mean_pathfinder*100:.1f}%")

        # Per-model |combined - pathfinder| (robustness)
        diffs = [abs(s["combined"] - s["pathfinder"]) for _, s in valid]
        print(f"  mean |combined - pathfinder| = {sum(diffs)/n*100:.1f}%")
        print(f"  max  |combined - pathfinder| = {max(diffs)*100:.1f}%")
        pf_eq = sum(1 for _, s in valid if abs(s["combined"] - s["pathfinder"]) <= 1e-9)
        print(f"  models with combined SR == pathfinder SR: {pf_eq} / {n}")

    # LaTeX rows
    print("\n% --- LaTeX rows for tab:combined-proxy ---")
    for i, (tier_name, models) in enumerate(MODEL_TIERS):
        if i > 0:
            print(r"\addlinespace[2pt]")
        print(rf"\rowcolor{{gray!15}}\multicolumn{{6}}{{l}}{{\textit{{{tier_name}}}}} \\")
        for mid, display in models:
            md = BASE / mid
            cells = [display]
            for key, rel in TESTS:
                v = per_test_sr(md, rel)
                cells.append(r"N/A" if v is None else rf"{v*100:.0f}\%")
            print(" & ".join(cells) + r" \\")


if __name__ == "__main__":
    main()
