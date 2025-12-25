Here is a **short, honest README** that explains *your* approach clearly, without overclaiming it as full Pratt parsing.

---

# Simple Expression Parser (Pratt-Inspired)

This project implements a **two-pass expression parser** inspired by Pratt parsing, designed to evaluate arithmetic expressions containing:

* Integers
* Binary operators: `+ - * /`
* Correct operator precedence (`*` and `/` before `+` and `-`)

The implementation is intentionally simple and focuses on **operator grouping rather than full binding-power rules**.

---

## Overview of the Approach

Instead of using a full Pratt parser or recursive descent grammar, this parser works in **two main stages**:

### 1. Tokenization

The input string is first converted into a list of tokens:

* Numbers (e.g. `"53"`)
* Operators (e.g. `"*"`, `"+"`)

Example:

```text
"53*4+7*6"
→ ["53", "*", "4", "+", "7", "*", "6"]
```

---

### 2. Operator Grouping (Precedence Handling)

To handle operator precedence, the parser performs a **grouping pass**:

* Multiplication (`*`) and division (`/`) are grouped into `Expression` nodes immediately.
* Addition (`+`) and subtraction (`-`) are left for a later evaluation pass.

Example grouping result:

```text
53 * 4 + 7 * 6
→ [Expr(53 * 4), '+', Expr(7 * 6)]
```

This mirrors Pratt parsing’s idea of **higher-precedence operators binding tighter**, but without explicit binding powers.

---

### 3. Evaluation

Evaluation happens in two layers:

* Nested `Expression` nodes are evaluated recursively.
* The remaining flat sequence is evaluated left-to-right using `+` and `-`.

This ensures correct precedence while keeping the logic easy to reason about.

---

## Why This Is “Pratt-Inspired”

* Higher-precedence operators are handled first
* Expressions are represented as an AST
* Evaluation is recursive

However:

* There are no explicit binding powers
* Parsing is done via grouping rather than recursive token consumption

This makes it **simpler than a true Pratt parser**, but still structurally correct for basic arithmetic.

---

## Limitations

* No parentheses support (yet)
* No unary operators
* Assumes valid input
* Designed for learning, not production parsing

---

## Goal

The goal of this project is to **understand operator precedence and AST evaluation** without jumping straight into a complex parsing framework.

It serves as a stepping stone toward:

* Full Pratt parsing
* Recursive descent parsers
* Expression grammars

---

