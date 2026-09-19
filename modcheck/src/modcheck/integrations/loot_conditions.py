"""Three-valued evaluator for LOOT condition strings.

LOOT gates much of its metadata behind conditions such as
``file("x.esp") and not checksum("y.esp", 0DEADBEEF)``. Evaluating these
against a player's *actual* configuration is what turns a generic rule into a
statement about them.

The important part is the third value. If we do not know whether a file is
installed, the answer is UNKNOWN -- not false. Collapsing unknown to false is
how a tool ends up confidently reporting a missing requirement that is in fact
installed, or a clean bill of health it has not earned.

Grammar (from the libloot metadata documentation):

    expression: condition, { "or", compound_condition }
    compound_condition: condition, { "and", condition }
    condition: [ "not" ], ( function | "(" expression ")" )
"""
from __future__ import annotations

import re
from dataclasses import dataclass
from typing import Any, Callable

TRUE = True
FALSE = False
UNKNOWN = None
Tri = bool | None

# A path argument is a regex if it contains any of these characters.
REGEX_CHARS = set(":\\*?|")

_TOKEN = re.compile(r"""
    \s*(?:
      (?P<lparen>\()
    | (?P<rparen>\))
    | (?P<and>\band\b)
    | (?P<or>\bor\b)
    | (?P<not>\bnot\b)
    | (?P<func>[A-Za-z_][A-Za-z_0-9]*)\s*\((?P<args>[^()]*)\)
    )""", re.VERBOSE)

_ARG = re.compile(r'"([^"]*)"|([^,\s][^,]*)')


class ConditionError(ValueError):
    """The condition string could not be parsed."""


def tri_not(v: Tri) -> Tri:
    return UNKNOWN if v is UNKNOWN else not v


def tri_and(a: Tri, b: Tri) -> Tri:
    if a is FALSE or b is FALSE:
        return FALSE
    if a is UNKNOWN or b is UNKNOWN:
        return UNKNOWN
    return TRUE


def tri_or(a: Tri, b: Tri) -> Tri:
    if a is TRUE or b is TRUE:
        return TRUE
    if a is UNKNOWN or b is UNKNOWN:
        return UNKNOWN
    return FALSE


@dataclass
class Call:
    name: str
    args: list[str]


def _split_args(raw: str) -> list[str]:
    out = []
    for m in _ARG.finditer(raw):
        out.append((m.group(1) if m.group(1) is not None else m.group(2)).strip())
    return out


def tokenize(text: str) -> list[Any]:
    tokens: list[Any] = []
    pos = 0
    while pos < len(text):
        if text[pos].isspace():
            pos += 1
            continue
        m = _TOKEN.match(text, pos)
        if not m or m.end() == m.start():
            raise ConditionError(f"cannot parse condition at offset {pos}: {text[pos:pos + 30]!r}")
        pos = m.end()
        if m.group("func"):
            tokens.append(Call(m.group("func"), _split_args(m.group("args") or "")))
        else:
            for kind in ("lparen", "rparen", "and", "or", "not"):
                if m.group(kind):
                    tokens.append(kind)
                    break
    if not tokens:
        raise ConditionError("empty condition")
    return tokens


class _Parser:
    """Recursive descent over the token list, evaluating as it goes."""

    def __init__(self, tokens: list[Any], call: Callable[[Call], Tri]) -> None:
        self.tokens = tokens
        self.pos = 0
        self.call = call

    def peek(self) -> Any:
        return self.tokens[self.pos] if self.pos < len(self.tokens) else None

    def take(self) -> Any:
        tok = self.peek()
        self.pos += 1
        return tok

    def expression(self) -> Tri:
        value = self.compound()
        while self.peek() == "or":
            self.take()
            value = tri_or(value, self.compound())
        return value

    def compound(self) -> Tri:
        value = self.condition()
        while self.peek() == "and":
            self.take()
            value = tri_and(value, self.condition())
        return value

    def condition(self) -> Tri:
        negate = False
        while self.peek() == "not":
            self.take()
            negate = not negate
        tok = self.take()
        if tok == "lparen":
            value = self.expression()
            if self.take() != "rparen":
                raise ConditionError("unbalanced parentheses")
        elif isinstance(tok, Call):
            value = self.call(tok)
        else:
            raise ConditionError(f"unexpected token {tok!r}")
        return tri_not(value) if negate else value


def evaluate(condition: str, resolver: Callable[[Call], Tri]) -> Tri:
    """Evaluate a condition string. Returns True, False or None (unknown)."""
    parser = _Parser(tokenize(condition), resolver)
    value = parser.expression()
    if parser.pos != len(parser.tokens):
        raise ConditionError(f"trailing tokens in condition: {condition!r}")
    return value


def is_regex(path: str) -> bool:
    return any(c in REGEX_CHARS for c in path)


def compile_path_regex(path: str) -> re.Pattern[str]:
    """LOOT anchors filename regexes and evaluates them case-insensitively.

    Only the final path component is a regex; the directory part is literal.
    Backslashes are NOT separators here: the specification requires regex paths
    to use ``/``, so a backslash is always a regex escape.
    """
    head, _, tail = path.rpartition("/")
    pattern = f"^{tail}$"
    if head:
        pattern = f"^{re.escape(head)}/(?:{tail})$"
    return re.compile(pattern, re.IGNORECASE)
