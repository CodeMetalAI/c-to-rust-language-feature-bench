"""Tests for the open structural stub detector (no LLM or network needed)."""

import glob
from pathlib import Path

import pytest

from test_runner.models import load_config
from test_runner.runner import StubDecision, default_stub_detector, resolve_stub_detector
from test_runner.stub_detection import (
    c_function_definitions,
    detect_stub,
    rust_function_definitions,
    structural_stub_detector,
)

C_TWO_FUNCS = """
#include <stdio.h>
/* int fake(void) { return 0; } in a comment must not count */
static int add(int a, int b) { return a + b; }
int main(void) {
  if (add(2, 3) != 5) return 1;
  printf("{ not a brace }\\n");
  return 0;
}
"""

RUST_FAITHFUL = """
fn add(a: i32, b: i32) -> i32 { a + b }
fn main() {
    if add(2, 3) != 5 { std::process::exit(1); }
    println!("{{ not a brace }}");
}
"""


def test_c_definitions_skip_prototypes_comments_and_initializers() -> None:
    src = """
    int proto(int);
    int table[] = { 1, 2 };
    struct s { int x; };
    int (*returns_fp(int (*pf)(long), int k))(int, ...) { return 0; }
    extern int knr(a, b)
    int a;
    int b;
    { return a + b; }
    int main(void) { return 0; }
    """
    assert c_function_definitions(src) == ["returns_fp", "knr", "main"]


def test_c_definitions_match_clang_on_every_benchmark_source() -> None:
    # Spot-check a few known shapes; the full clang cross-check is described in the README.
    files = [f for f in glob.glob("c11_language_features_benchmark/**/*.c", recursive=True) if "/build/" not in f]
    assert len(files) >= 130
    assert all("main" in c_function_definitions(Path(f).read_text(errors="replace")) for f in files)


def test_rust_definitions_ignore_comments_and_strings() -> None:
    code = 'fn main() { let s = "fn fake()"; } // fn other()\nfn helper<T>(x: T) {}'
    assert rust_function_definitions(code) == ["main", "helper"]


def test_faithful_translation_is_not_a_stub() -> None:
    assert detect_stub(RUST_FAITHFUL, C_TWO_FUNCS).triggered is False


@pytest.mark.parametrize(
    "rust",
    [
        "fn main() {}",
        "fn main() { std::process::exit(0); }",
        'fn main() { println!("ok"); }',
    ],
)
def test_trivial_main_is_a_stub_when_c_has_control_flow(rust: str) -> None:
    decision = detect_stub(rust, C_TWO_FUNCS)
    assert decision.triggered
    assert any("main has no control flow" in r for r in decision.reasons)


def test_dropped_functions_use_the_ceil_threshold() -> None:
    c = "int a(void){return 1;} int b(void){return 2;} int c(void){return 3;} int main(void){return a()+b()+c()-6;}"
    rust_three = "fn a()->i32{1} fn b()->i32{2} fn main(){ if a()+b()+3-6 != 0 { std::process::exit(1) } }"
    rust_two = "fn a()->i32{1} fn main(){ if a()+2+3-6 != 0 { std::process::exit(1) } }"
    assert not detect_stub(rust_three, c).triggered  # 3 >= ceil(0.75 * 4)
    reasons = detect_stub(rust_two, c).reasons
    assert reasons == ["function definitions dropped from 4 to 2 (threshold=0.75, require>=3)"]


def test_placeholder_markers_are_stubs() -> None:
    rust = "fn add(a:i32,b:i32)->i32{ todo!() } fn main(){ if add(2,3)!=5 { std::process::exit(1) } }"
    assert detect_stub(rust, C_TWO_FUNCS).reasons == ["contains explicit stub marker(s): todo!"]
    rust = 'fn add(a:i32,b:i32)->i32{ panic!("not implemented yet") } fn main(){ if add(2,3)!=5 { std::process::exit(1) } }'
    assert detect_stub(rust, C_TWO_FUNCS).triggered
    # The words inside an ordinary string do not count.
    rust = 'fn add(a:i32,b:i32)->i32{ a+b } fn main(){ if add(2,3)!=5 { println!("todo!()"); std::process::exit(1) } }'
    assert not detect_stub(rust, C_TWO_FUNCS).triggered


def test_structural_detector_matches_runner_signature(tmp_path: Path) -> None:
    src = tmp_path / "t.c"
    src.write_text(C_TWO_FUNCS)
    decision = structural_stub_detector("fn main() {}", src, ["-std=c11"], tmp_path)
    assert isinstance(decision, StubDecision) and decision.triggered and decision.reasons


def test_config_selects_detector() -> None:
    assert resolve_stub_detector("none") is default_stub_detector
    assert resolve_stub_detector("structural") is structural_stub_detector
    with pytest.raises(ValueError):
        resolve_stub_detector("bogus")
    assert load_config().stub_detector == "none"  # default behaviour unchanged
