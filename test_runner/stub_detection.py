"""Open-source structural stub detector for generated Rust.

Follows the stub-detection criteria described in Appendix A-C of the paper: compare the
generated Rust against a structural baseline derived from the C source, and look for common
stub patterns. It is a dependency-free, best-effort reimplementation, not Code Metal's
proprietary detector. ``paper_experimental_results/scripts/compare_stub_detector.py`` measures
how often the two agree on the published results.

Checks (any one triggers):

* explicit placeholder markers: ``todo!``, ``unimplemented!``, or ``panic!``/``unreachable!``
  whose message says the code is not implemented;
* function definitions dropped: the C program defines ``n >= 1`` functions and the Rust
  program defines fewer than ``ceil(0.75 * n)``;
* trivial ``main``: the C ``main`` has control flow, but the Rust ``main`` has none, calls no
  local function, and only prints or exits with a literal (the "literal-only return" and
  "print-only program" patterns).
"""

from __future__ import annotations

import math
import re
from dataclasses import dataclass
from pathlib import Path

FUNCTION_DROP_THRESHOLD = 0.75

_C_KEYWORDS_NOT_FUNCTIONS = {"if", "for", "while", "switch", "return", "sizeof", "_Alignof", "_Generic", "do"}
_CONTROL_FLOW_C = re.compile(r"\b(if|for|while|switch|do|goto)\b|\?")
_CONTROL_FLOW_RUST = re.compile(r"\b(if|for|while|loop|match)\b|\?")
_PLACEHOLDER_MACRO = re.compile(r"\b(todo|unimplemented)!\s*[\(\[\{]")
_PLACEHOLDER_PANIC = re.compile(
    r"\b(panic|unreachable)!\s*\(\s*\"[^\"]*(not\s+(yet\s+)?implemented|todo|stub|placeholder)[^\"]*\"",
    re.IGNORECASE,
)
_RUST_FN = re.compile(r"\bfn\s+([A-Za-z_][A-Za-z0-9_]*)\s*(<[^>{}]*>)?\s*\(")
_PRINT_OR_EXIT_STMT = re.compile(
    r"^(?:std::)?(?:process::)?exit\s*\(\s*-?\d+\s*\)$"
    r"|^(?:print|println|eprint|eprintln)!\s*\(.*\)$"
    r"|^-?\d+$"
    r"|^return(?:\s+-?\d+)?$"
    r"|^\(\)$",
    re.DOTALL,
)


@dataclass
class StructuralDecision:
    triggered: bool
    reasons: list[str]


def strip_c(code: str) -> str:
    """Remove comments, string/char literals and preprocessor lines from C source."""
    code = re.sub(r"/\*.*?\*/", " ", code, flags=re.DOTALL)
    code = re.sub(r"//[^\n]*", " ", code)
    code = re.sub(r'"(?:\\.|[^"\\\n])*"', '""', code)
    code = re.sub(r"'(?:\\.|[^'\\\n])*'", "''", code)
    lines = []
    continuing = False
    for line in code.splitlines():
        if continuing or line.lstrip().startswith("#"):
            continuing = line.rstrip().endswith("\\")
            lines.append("")
            continue
        lines.append(line)
    return "\n".join(lines)


def strip_rust(code: str) -> str:
    """Remove comments and string literals from Rust source (raw strings included)."""
    code = re.sub(r"/\*.*?\*/", " ", code, flags=re.DOTALL)
    code = re.sub(r"//[^\n]*", " ", code)
    code = re.sub(r'r(#*)"(?:.|\n)*?"\1', '""', code)
    code = re.sub(r'b?"(?:\\.|[^"\\])*"', '""', code)
    return code


_C_NON_NAME_WORDS = _C_KEYWORDS_NOT_FUNCTIONS | {
    "int",
    "char",
    "void",
    "double",
    "float",
    "long",
    "short",
    "unsigned",
    "signed",
    "struct",
    "union",
    "enum",
    "const",
    "volatile",
    "static",
    "extern",
    "inline",
    "_Noreturn",
    "restrict",
    "register",
    "auto",
    "_Bool",
    "_Atomic",
    "_Thread_local",
    "__attribute__",
}
_IDENT_CALL = re.compile(r"([A-Za-z_][A-Za-z0-9_]*)\s*\(")


def _definition_head(code: str, open_brace: int) -> str | None:
    """Declarator text before a file-scope ``{`` if it opens a function body, else None."""
    start = max(code.rfind(";", 0, open_brace), code.rfind("}", 0, open_brace)) + 1
    head = code[start:open_brace]
    if head.strip().endswith(")"):
        return head
    # K&R definition: ``int g(a, b) int a; int b; {``. Walk back over parameter declarations.
    prev_close = code.rfind("}", 0, open_brace) + 1
    pieces = code[prev_close:open_brace].split(";")
    if len(pieces) < 2 or pieces[-1].strip():
        return None
    k = len(pieces) - 2
    while k >= 0 and "(" not in pieces[k] and pieces[k].strip():
        k -= 1
    if k < 0:
        return None
    # The first parameter declaration shares a piece with the declarator: "int g(a, b) int a".
    close = pieces[k].rfind(")")
    if close == -1 or "(" in pieces[k][close + 1 :]:
        return None
    return pieces[k][: close + 1]


def c_function_definitions(c_source: str) -> list[str]:
    """Names of functions defined (not just declared) at file scope in C source.

    Handles ordinary, K&R-style and pointer-returning declarators
    (``int (*f(int))(long) {``). Checked against clang's AST on all benchmark sources.
    """
    code = strip_c(c_source)
    names: list[str] = []
    depth = 0
    for i, ch in enumerate(code):
        if ch == "{":
            if depth == 0:
                head = _definition_head(code, i)
                if head is not None and "=" not in head:
                    name = next(
                        (m.group(1) for m in _IDENT_CALL.finditer(head) if m.group(1) not in _C_NON_NAME_WORDS),
                        None,
                    )
                    if name is not None:
                        names.append(name)
            depth += 1
        elif ch == "}":
            depth = max(0, depth - 1)
    return names


def _body_after(code: str, open_brace: int) -> str:
    depth = 0
    for j in range(open_brace, len(code)):
        if code[j] == "{":
            depth += 1
        elif code[j] == "}":
            depth -= 1
            if depth == 0:
                return code[open_brace + 1 : j]
    return code[open_brace + 1 :]


def c_main_body(c_source: str) -> str | None:
    code = strip_c(c_source)
    m = re.search(r"\bmain\s*\([^)]*\)\s*\{", code)
    return _body_after(code, m.end() - 1) if m else None


def rust_function_definitions(rust_code: str) -> list[str]:
    return [m.group(1) for m in _RUST_FN.finditer(strip_rust(rust_code))]


def rust_main_body(rust_code: str) -> str | None:
    code = strip_rust(rust_code)
    m = re.search(r"\bfn\s+main\s*\(\s*\)[^{]*\{", code)
    return _body_after(code, m.end() - 1) if m else None


def _split_statements(body: str) -> list[str]:
    stmts, depth, start = [], 0, 0
    for k, ch in enumerate(body):
        if ch in "([{":
            depth += 1
        elif ch in ")]}":
            depth -= 1
        elif ch == ";" and depth == 0:
            stmts.append(body[start:k].strip())
            start = k + 1
    tail = body[start:].strip()
    if tail:
        stmts.append(tail)
    return [s for s in stmts if s]


def _is_trivial_main(rust_code: str, rust_body: str, rust_functions: list[str]) -> bool:
    if _CONTROL_FLOW_RUST.search(rust_body):
        return False
    helpers = [f for f in rust_functions if f != "main"]
    if any(re.search(rf"\b{re.escape(f)}\s*(::\s*<[^>]*>\s*)?\(", rust_body) for f in helpers):
        return False
    return all(_PRINT_OR_EXIT_STMT.match(s) for s in _split_statements(rust_body))


def detect_stub(rust_code: str, c_source: str) -> StructuralDecision:
    """Return whether ``rust_code`` looks like a semantically incomplete stub of ``c_source``."""
    reasons: list[str] = []
    stripped_rust = strip_rust(rust_code)

    markers = sorted({m.group(1) + "!" for m in _PLACEHOLDER_MACRO.finditer(stripped_rust)})
    if _PLACEHOLDER_PANIC.search(rust_code):
        markers.append("panic!/unreachable! with a not-implemented message")
    if markers:
        reasons.append("contains explicit stub marker(s): " + ", ".join(markers))

    c_functions = c_function_definitions(c_source)
    rust_functions = rust_function_definitions(rust_code)
    if len(c_functions) >= 1:
        required = math.ceil(FUNCTION_DROP_THRESHOLD * len(c_functions))
        if len(rust_functions) < required:
            reasons.append(
                f"function definitions dropped from {len(c_functions)} to {len(rust_functions)} "
                f"(threshold={FUNCTION_DROP_THRESHOLD}, require>={required})"
            )

    c_body = c_main_body(c_source)
    rust_body = rust_main_body(rust_code)
    if (
        c_body is not None
        and rust_body is not None
        and _CONTROL_FLOW_C.search(c_body)
        and _is_trivial_main(rust_code, rust_body, rust_functions)
    ):
        reasons.append("main has no control flow and only prints or exits with a literal")

    return StructuralDecision(triggered=bool(reasons), reasons=reasons)


def structural_stub_detector(
    rust_code: str,
    source_file_path: Path,
    source_compile_args: list[str],
    sample_dir: Path,
):
    """Drop-in ``stub_detector`` for :mod:`test_runner.runner` (same signature)."""
    from .runner import StubDecision  # local import: runner imports this module

    decision = detect_stub(rust_code, Path(source_file_path).read_text(errors="replace"))
    return StubDecision(triggered=decision.triggered, reasons=decision.reasons or None)
