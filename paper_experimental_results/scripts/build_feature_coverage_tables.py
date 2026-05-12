"""
Build fig2_feature_coverage.tex from temp 0.2 data.

Reads per-test result.json files under
    ../2026-05-06T18-03-03/
and writes LaTeX table(s) to scripts/tables/.

Aggregation: pass@5 — a test counts as passed if at least one of the 5 samples
in result.json["samples"] has match == True.

Palette: Okabe-Ito / Wong colorblind-safe diverging scale
    none pass → vermillion #D55E00
    half      → cream     #F0F0F0
    all pass  → blue      #0072B2

No data files are written — output is .tex only.
"""

import json
import numpy as np
from pathlib import Path
from collections import defaultdict

# ── paths ────────────────────────────────────────────────────────────────────
SCRIPT_DIR = Path(__file__).resolve().parent  # scripts/
REPO = SCRIPT_DIR.parent  # paper_experimental_results/
DATA_ROOT = REPO / "2026-05-06T18-03-03"
OUT_DIR = SCRIPT_DIR / "tables"

# ── model layout (gemma-3-12b replaces gemma-2-9b) ──────────────────────────
MODEL_GROUPS = [
    ("Prog.-Spec.", [
        ("gpt-5.2-codex",            "Codex"),
        ("grok-code-fast-1",          "Grok"),
        ("mistral-codestral-2508",    "Codestral"),
    ]),
    ("Flagship", [
        ("claude-opus-4.5",           "Opus"),
        ("claude-sonnet-4.5",         "Sonnet"),
        ("gpt-4-turbo",               "GPT-4T"),
        ("gpt-4o",                    "GPT-4o"),
    ]),
    ("Large", [
        ("deepseek-v3.2",             "DeepSeek"),
        ("llama-3.1-70b",             "L3-70B"),
    ]),
    ("Small/LW", [
        ("gemma-3-12b",               "Gemma 3"),
        ("llama-3.1-8b-instruct",     "L3-8B"),
        ("gpt-4o-mini",               "4o-mini"),
    ]),
]
MODEL_ORDER  = [m for _, grp in MODEL_GROUPS for m, _ in grp]
MODEL_LABELS = [l for _, grp in MODEL_GROUPS for _, l in grp]

# ── one combined table covering all clauses in C11 section order ─────────────
ALL_CATS = ["concepts", "conversions", "lex", "expr",
            "decl", "stmt", "extern", "preprocess"]
SPLIT = [
    ("", ALL_CATS,
       r"across the benchmark"),
]

CAT_LABELS = {
    "concepts":    r"Concepts \textnormal{(\S6.2.4)}",
    "conversions": r"Conversions \textnormal{(\S6.3)}",
    "lex":         r"Lexical Elements \textnormal{(\S6.4)}",
    "expr":        r"Expressions \textnormal{(\S6.5)}",
    "decl":        r"Declarations \textnormal{(\S6.7)}",
    "stmt":        r"Statements \textnormal{(\S6.8)}",
    "extern":      r"External Definitions \textnormal{(\S6.9)}",
    "preprocess":  r"Preprocessing \textnormal{(\S6.10)}",
}

FEATURE_NAMES = {
    "concepts.storage_duration":       "Storage duration",
    "conversions.bool_char_int":       "Bool/char/int conv.",
    "conversions.lvalue_array_fun":    "Lvalue/array/function conv.",
    "conversions.pointers":            "Pointer conversions",
    "decl.align_specifiers":           "Alignment specifiers",
    "decl.array_decl":                 "Array declarators",
    "decl.atm_type_specifiers":        "Atomic type specifiers",
    "decl.enumeration":                "Enumeration",
    "decl.fun_decl":                   "Function declarators",
    "decl.function_specifiers":        "Function specifiers",
    "decl.initialization":             "Initialization",
    "decl.ptr_decl":                   "Pointer declarators",
    "decl.restrict":                   "Restrict qualifier",
    "decl.structure_and_union":        "Structures and unions",
    "decl.tags":                       "Tags",
    "decl.type_defs":                  "Typedef",
    "decl.type_names":                 "Type names",
    "decl.type_qualif":                "Type qualifiers",
    "expr.add":                        "Additive operators",
    "expr.assign.simple":              "Simple assignment",
    "expr.bitwise.shift":              "Bitwise shift",
    "expr.comma":                      "Comma operator",
    "expr.cond":                       "Conditional operator",
    "expr.eq":                         "Equality operators",
    "expr.generic":                    "Generic selection",
    "expr.logical.and":                "Logical AND",
    "expr.logical.or":                 "Logical OR",
    "expr.mul":                        "Multiplicative operators",
    "expr.postfix.call":               "Function calls",
    "expr.postfix.compound_literal":   "Compound literals",
    "expr.postfix.incdec":             "Postfix inc/dec",
    "expr.postfix.member":             "Member access",
    "expr.postfix.subscript":          "Array subscripting",
    "expr.rel":                        "Relational operators",
    "expr.unary.addr_indir":           r"Address \& indirection",
    "expr.unary.incdec":               "Prefix inc/dec",
    "expr.unary.sizeof_alignof":       "sizeof/alignof",
    "extern.extern_objs":              "External object defs.",
    "extern.fun_defs":                 "Function definitions",
    "lex.ident":                       "Predefined identifiers",
    "preprocess.dhash_op":             r"\#\# operator",
    "preprocess.macro_scope":          "Macro scope",
    "stmt.expr_null":                  r"Expression \& null stmts",
    "stmt.goto":                       "Goto statement",
    "stmt.return":                     "Return statement",
    "stmt.select.switch":              "Switch statement",
}

# ── Okabe-Ito / Wong colorblind-safe palette ────────────────────────────────
LOW  = np.array([213,  94,   0])   # vermillion  #D55E00
MID  = np.array([240, 240, 240])   # near-white  #F0F0F0
HIGH = np.array([  0, 114, 178])   # blue        #0072B2
GREY = "DDDDDD"

HEX_LOW  = "%02X%02X%02X" % tuple(LOW)
HEX_HIGH = "%02X%02X%02X" % tuple(HIGH)


def hex_color(frac: float) -> str:
    if frac is None or (isinstance(frac, float) and np.isnan(frac)):
        return GREY
    f = float(np.clip(frac, 0.0, 1.0))
    rgb = LOW + 2 * f * (MID - LOW) if f <= 0.5 else MID + 2 * (f - 0.5) * (HIGH - MID)
    return "%02X%02X%02X" % tuple(int(round(x)) for x in rgb)


def feat_cat(fid: str) -> str:
    return fid.split(".", 1)[0]


# ── aggregate: pass@5 per (model, feature) ──────────────────────────────────
def aggregate() -> dict:
    """Returns {model: {feature: (passed, total)}}."""
    by_model = {}
    for model_dir in sorted(DATA_ROOT.iterdir()):
        if not model_dir.is_dir():
            continue
        counts = defaultdict(lambda: [0, 0])  # feature -> [passed, total]
        for result_path in model_dir.rglob("result.json"):
            r = json.loads(result_path.read_text())
            feature = r.get("primary_feature")
            if not feature:
                continue
            samples = r.get("samples", [])
            passed = any(s.get("match") for s in samples)
            counts[feature][1] += 1
            if passed:
                counts[feature][0] += 1
        by_model[model_dir.name] = {f: tuple(v) for f, v in counts.items()}
    return by_model


# ── LaTeX assembly ──────────────────────────────────────────────────────────
N_MODELS = len(MODEL_ORDER)
GROUP_SIZES = [len(grp) for _, grp in MODEL_GROUPS]
COL_SPEC = ("@{}l"
            + "r" * GROUP_SIZES[0]
            + "".join(f"|{'r' * s}" for s in GROUP_SIZES[1:])
            + "@{}")


def group_header_row() -> str:
    cells = [""]
    for label, grp in MODEL_GROUPS:
        cells.append(rf"\multicolumn{{{len(grp)}}}{{c}}{{\textbf{{{label}}}}}")
    return " & ".join(cells) + r" \\"


def cmidrules_row() -> str:
    parts = []
    col = 2
    for _, grp in MODEL_GROUPS:
        n = len(grp)
        parts.append(rf"\cmidrule(lr){{{col}-{col + n - 1}}}")
        col += n
    return " ".join(parts)


def model_label_row() -> str:
    cells = [r"\textbf{Feature}"]
    cells += [rf"\textbf{{\scriptsize {l}}}" for l in MODEL_LABELS]
    return " & ".join(cells) + r" \\"


def feature_order(by_model: dict, cats: list) -> list:
    """Sort: category in given order, then by mean pass-rate descending."""
    features = set()
    for m in MODEL_ORDER:
        features |= set(by_model.get(m, {}).keys())
    features = [f for f in features if feat_cat(f) in cats]

    cat_rank = {c: i for i, c in enumerate(cats)}

    def mean_frac(f):
        fracs = []
        for m in MODEL_ORDER:
            pt = by_model.get(m, {}).get(f)
            if pt and pt[1] > 0:
                fracs.append(pt[0] / pt[1])
        return float(np.mean(fracs)) if fracs else 0.0

    return sorted(features,
                  key=lambda f: (cat_rank.get(feat_cat(f), 99), -mean_frac(f), f))


def data_row(fid: str, by_model: dict) -> str:
    cells = [FEATURE_NAMES.get(fid, fid)]
    for m in MODEL_ORDER:
        pt = by_model.get(m, {}).get(fid)
        if pt is None or pt[1] == 0:
            cells.append(rf"\ccol{{{GREY}}}{{---}}")
        else:
            p, t = pt
            frac = p / t
            cells.append(rf"\ccol{{{hex_color(frac)}}}{p}/{t}")
    return " & ".join(cells) + r" \\"


def build_table(suffix: str, cats: list, desc: str, by_model: dict) -> str:
    feats = feature_order(by_model, cats)

    L = [f"% Auto-generated — table {suffix.upper()} ({len(feats)} features, pass@5, temp 0.2)",
         r"\begin{table*}[!p]",
         r"\centering\scriptsize",
         r"\renewcommand{\arraystretch}{0.85}",
         r"\setlength{\tabcolsep}{3pt}",
         r"\resizebox{\linewidth}{!}{%",
         rf"\begin{{tabular}}{{{COL_SPEC}}}",
         r"\toprule",
         group_header_row(),
         cmidrules_row(),
         model_label_row(),
         r"\midrule"]

    prev_cat = None
    for fid in feats:
        cat = feat_cat(fid)
        if cat != prev_cat:
            if prev_cat is not None:
                L.append(r"\addlinespace[2pt]")
            L.append(rf"\rowcolor{{gray!15}}"
                     rf"\multicolumn{{{N_MODELS + 1}}}{{l}}"
                     rf"{{\textit{{{CAT_LABELS[cat]}}}}} \\")
            prev_cat = cat
        L.append(data_row(fid, by_model))

    L.append(r"\bottomrule")
    L.append(r"\end{tabular}}%")
    L.append(rf"\caption{{\textbf{{Per-feature translation consistency $\mathrm{{TC}}(f, m)$ {desc}.}} "
             rf"Each cell shows the number of tests successfully translated "
             rf"out of the total tests exercising that feature; a test is "
             rf"counted as successfully translated if at least one of five "
             rf"independent samples drawn at temperature~0.2 produces a "
             rf"correct, safe-Rust translation. "
             rf"Color: \textcolor[HTML]{{{HEX_HIGH}}}{{\rule{{0.7em}}{{0.7em}}}}~all tests pass to "
             rf"\textcolor[HTML]{{{HEX_LOW}}}{{\rule{{0.7em}}{{0.7em}}}}~no tests pass.}}")
    label = f"tab:feature_coverage_{suffix}" if suffix else "tab:feature_coverage"
    L.append(rf"\label{{{label}}}")
    L.append(r"\end{table*}")
    return "\n".join(L) + "\n"


def main():
    by_model = aggregate()

    # sanity checks
    expected_models = {m for _, grp in MODEL_GROUPS for m, _ in grp}
    missing = expected_models - by_model.keys()
    assert not missing, f"missing model dirs: {missing}"
    assert "gemma-3-12b" in by_model, "gemma-3-12b missing from data"
    assert "gemma-2-9b" not in by_model, "gemma-2-9b unexpectedly present"

    feature_counts = {m: len(feats) for m, feats in by_model.items()}
    print(f"Models found: {len(by_model)}  "
          f"(features per model: {sorted(set(feature_counts.values()))})")

    OUT_DIR.mkdir(parents=True, exist_ok=True)
    for suffix, cats, desc in SPLIT:
        filename = f"fig2_feature_coverage_{suffix}.tex" if suffix else "fig2_feature_coverage.tex"
        path = OUT_DIR / filename
        path.write_text(build_table(suffix, cats, desc, by_model))
        feats = feature_order(by_model, cats)
        print(f"wrote {path.name}  ({len(feats)} rows)")


if __name__ == "__main__":
    main()
