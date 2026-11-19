"""Safe evaluator for the rule expressions used by POST /rules/eval.

The endpoint used to run each rule with eval(). Passing {} as the globals does
not sandbox eval, because Python adds __builtins__ to an empty globals dict.
So a rule such as __import__('os').system('...') ran any command on the server.

This module never calls eval or exec. It parses a rule into a Python syntax
tree, checks every node against a short allowlist, and then walks the tree
itself. A rule can only use:

  * the field names the caller allows (for example cpu_usage, memory, cmd, name)
  * strings, numbers, True, False and None
  * and, or, not
  * ==, !=, <, <=, >, >=, in, not in
  * +, -, *, / on numbers only
  * lists, tuples and sets, for membership tests
  * a few string methods: lower, upper, strip, startswith, endswith

Everything else (imports, attribute access, indexing, function calls,
lambdas, comprehensions, dunder names) is rejected before the rule runs.
"""

import ast
import operator

MAX_RULE_LENGTH = 500
MAX_DEPTH = 30

STRING_METHODS = frozenset({"lower", "upper", "strip", "startswith", "endswith"})

_COMPARE_OPS = {
    ast.Eq: operator.eq,
    ast.NotEq: operator.ne,
    ast.Lt: operator.lt,
    ast.LtE: operator.le,
    ast.Gt: operator.gt,
    ast.GtE: operator.ge,
    ast.In: lambda left, right: left in right,
    ast.NotIn: lambda left, right: left not in right,
}

_ARITH_OPS = {
    ast.Add: operator.add,
    ast.Sub: operator.sub,
    ast.Mult: operator.mul,
    ast.Div: operator.truediv,
}

_CONSTANT_TYPES = (str, int, float, bool, type(None))


class RuleError(ValueError):
    """A rule is not allowed, or it failed while running."""


def compile_rule(text, allowed_names):
    """Parse and check a rule. Returns a tree that evaluate() can run.

    Raises RuleError if the rule uses anything outside the allowlist.
    """
    if not isinstance(text, str) or not text.strip():
        raise RuleError("rule must be a non-empty string")
    if len(text) > MAX_RULE_LENGTH:
        raise RuleError("rule is longer than %d characters" % MAX_RULE_LENGTH)
    try:
        tree = ast.parse(text.strip(), mode="eval")
    except SyntaxError as error:
        raise RuleError("syntax error: %s" % error.msg) from None
    _check(tree.body, frozenset(allowed_names), 0)
    return tree.body


def _check(node, names, depth):
    if depth > MAX_DEPTH:
        raise RuleError("rule is nested too deeply")
    depth = depth + 1

    if isinstance(node, ast.Constant):
        if not isinstance(node.value, _CONSTANT_TYPES):
            raise RuleError("constant type not allowed: %s" % type(node.value).__name__)
        return

    if isinstance(node, ast.Name):
        if node.id not in names:
            raise RuleError("unknown field: %s" % node.id)
        return

    if isinstance(node, ast.BoolOp):
        for value in node.values:
            _check(value, names, depth)
        return

    if isinstance(node, ast.UnaryOp):
        if not isinstance(node.op, (ast.Not, ast.USub)):
            raise RuleError("operator not allowed: %s" % type(node.op).__name__)
        _check(node.operand, names, depth)
        return

    if isinstance(node, ast.BinOp):
        if type(node.op) not in _ARITH_OPS:
            raise RuleError("operator not allowed: %s" % type(node.op).__name__)
        _check(node.left, names, depth)
        _check(node.right, names, depth)
        return

    if isinstance(node, ast.Compare):
        for op in node.ops:
            if type(op) not in _COMPARE_OPS:
                raise RuleError("comparison not allowed: %s" % type(op).__name__)
        _check(node.left, names, depth)
        for comparator in node.comparators:
            _check(comparator, names, depth)
        return

    if isinstance(node, (ast.List, ast.Tuple, ast.Set)):
        for element in node.elts:
            _check(element, names, depth)
        return

    if isinstance(node, ast.Call):
        # Only a whitelisted string method called on a checked value,
        # like cmd.lower() or name.startswith("nc").
        func = node.func
        if not isinstance(func, ast.Attribute) or func.attr not in STRING_METHODS:
            raise RuleError("function calls are not allowed, only these string methods: "
                            + ", ".join(sorted(STRING_METHODS)))
        if node.keywords:
            raise RuleError("keyword arguments are not allowed")
        if len(node.args) > 1:
            raise RuleError("string methods take at most one argument here")
        _check(func.value, names, depth)
        for arg in node.args:
            _check(arg, names, depth)
        return

    raise RuleError("not allowed in a rule: %s" % type(node).__name__)


def evaluate(node, values):
    """Run a tree from compile_rule() with the given field values."""
    if isinstance(node, ast.Constant):
        return node.value

    if isinstance(node, ast.Name):
        return values[node.id]

    if isinstance(node, ast.BoolOp):
        if isinstance(node.op, ast.And):
            for value in node.values:
                if not evaluate(value, values):
                    return False
            return True
        for value in node.values:
            if evaluate(value, values):
                return True
        return False

    if isinstance(node, ast.UnaryOp):
        operand = evaluate(node.operand, values)
        if isinstance(node.op, ast.Not):
            return not operand
        _require_number(operand)
        return -operand

    if isinstance(node, ast.BinOp):
        left = evaluate(node.left, values)
        right = evaluate(node.right, values)
        # Numbers only. This blocks "cmd" * 10**9 style memory blowups.
        _require_number(left)
        _require_number(right)
        try:
            return _ARITH_OPS[type(node.op)](left, right)
        except ZeroDivisionError:
            raise RuleError("division by zero") from None

    if isinstance(node, ast.Compare):
        left = evaluate(node.left, values)
        for op, comparator in zip(node.ops, node.comparators):
            right = evaluate(comparator, values)
            try:
                if not _COMPARE_OPS[type(op)](left, right):
                    return False
            except TypeError as error:
                raise RuleError("cannot compare: %s" % error) from None
            left = right
        return True

    if isinstance(node, ast.List):
        return [evaluate(element, values) for element in node.elts]
    if isinstance(node, ast.Tuple):
        return tuple(evaluate(element, values) for element in node.elts)
    if isinstance(node, ast.Set):
        return {evaluate(element, values) for element in node.elts}

    if isinstance(node, ast.Call):
        target = evaluate(node.func.value, values)
        if not isinstance(target, str):
            raise RuleError("string methods only work on text fields")
        args = [evaluate(arg, values) for arg in node.args]
        for arg in args:
            if not isinstance(arg, (str, tuple)):
                raise RuleError("string method arguments must be text")
        return getattr(target, node.func.attr)(*args)

    # compile_rule() already rejected everything else.
    raise RuleError("not allowed in a rule: %s" % type(node).__name__)


def _require_number(value):
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise RuleError("arithmetic only works on numbers")
