"""Compare the open structural stub detector with the labels in the published results.

Every published sample records whether Code Metal's proprietary detector flagged it
(``stub_detected``). This re-runs test_runner.stub_detection on each saved target.rs and reports
agreement, plus the success rate (Table II/V definition) each model would get with the open
detector applied on top of the published verdicts.

    uv run python paper_experimental_results/scripts/compare_stub_detector.py
"""

from __future__ import annotations

import collections
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT))

from test_runner.stub_detection import detect_stub

RUNS = ["2026-05-06T18-03-03", "2026-05-10T00-24-35"]


def main() -> None:
    confusion: collections.Counter[tuple[bool, bool]] = collections.Counter()
    open_only_passes: list[str] = []
    for run in RUNS:
        run_dir = ROOT / "paper_experimental_results" / run
        temperature = json.loads((run_dir / "summary.json").read_text()).get("temperature")
        per_model: dict[str, list[float]] = collections.defaultdict(lambda: [0, 0.0, 0.0])
        for result_path in sorted(run_dir.glob("*/**/result.json")):
            test_dir = result_path.parent
            result = json.loads(result_path.read_text())
            c_source = (test_dir / "source.c").read_text(errors="replace")
            ok = ok_open = 0
            for sample in result["samples"]:
                rust_path = test_dir / f"sample_{sample['sample_index']:03d}" / "target.rs"
                if sample.get("provider_error") or not rust_path.exists():
                    continue
                flagged = detect_stub(rust_path.read_text(errors="replace"), c_source).triggered
                confusion[(bool(sample.get("stub_detected")), flagged)] += 1
                if sample.get("match"):
                    ok += 1
                    if flagged:
                        open_only_passes.append(str(rust_path.relative_to(ROOT)))
                    else:
                        ok_open += 1
            n = len(result["samples"])
            stats = per_model[result["llm_name"]]
            stats[0] += 1
            stats[1] += ok / n
            stats[2] += ok_open / n
        print(f"\n{run} (temperature {temperature})")
        print(f"  {'model':26s} {'SR published':>12s} {'SR +open detector':>18s}")
        for model, (tests, sr, sr_open) in sorted(per_model.items()):
            print(f"  {model:26s} {100 * sr / tests:11.1f}% {100 * sr_open / tests:17.1f}%")

    tp, fn = confusion[(True, True)], confusion[(True, False)]
    fp, tn = confusion[(False, True)], confusion[(False, False)]
    total = tp + fn + fp + tn
    print(f"\nSamples compared: {total}")
    print(f"  both flag: {tp}   proprietary only: {fn}   open only: {fp}   neither: {tn}")
    print(f"  agreement: {100 * (tp + tn) / total:.2f}%")
    print(f"\nPublished passes the open detector flags: {len(open_only_passes)}")
    for path in open_only_passes:
        print(f"  {path}")


if __name__ == "__main__":
    main()
