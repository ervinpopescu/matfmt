# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**matfmt** is a Rust rewrite of the [matlab-formatter-vscode](https://github.com/affenwiesel/matlab-formatter-vscode) Python-based MATLAB code formatter. It formats `.m` files by handling indentation for control structures, expression formatting with operator spacing, multi-line matrix/cell array alignment, and line continuation.

The original Python implementation is preserved in `original_project/matlab_formatter.py` for reference.

## Build & Test

```sh
cargo build          # build library + binary
cargo test           # run all tests (36 integration tests)
cargo run -- <file>  # format a file to stdout
cargo run -- -       # format stdin to stdout
```

### CLI Options

```
matfmt <file> [OPTIONS]
  --indent-width <N>              Spaces per indent level (default: 4)
  --indent-mode <MODE>            all-functions | only-nested | classic
  --operator-spacing <MODE>       all-operators | exclude-pow | no-spaces
  --matrix-indent <MODE>          aligned | simple
  --no-separate-blocks            Don't insert blank lines around control blocks
```

## Architecture

Four modules under `src/`:

- **`config.rs`** — `FormatterConfig` struct and enums (`IndentMode`, `OperatorSpacing`, `MatrixIndent`). All enums derive `clap::ValueEnum` for CLI integration.
- **`indent.rs`** — `IndentEngine` processes each line for control structure indentation. Owns all control-structure regexes (`if/for/while/switch/try/function/end/etc.`). Manages `istep`/`fstep` stacks and tracks block comments, line comments, ignore directives, and ellipsis line continuation.
- **`expression.rs`** — Recursive `extract()` pattern (ported from Python) that splits a line into `(before, token, after)` by matching operators, strings, comments, numbers, etc. Applies spacing rules based on `OperatorSpacing` config. Also provides `clean_strings_and_comments()` used by matrix tracking.
- **`matrix.rs`** — `MatrixTracker` tracks open/close `[]` and `{}` across lines. Computes alignment indent for continuation lines (aligned to bracket column or simple indent width).
- **`lib.rs`** — `Formatter` struct orchestrates: for each line, check matrix state, run indent engine, apply expression formatting, handle blank line separation.
- **`bin/main.rs`** — CLI binary using `clap` derive with all formatter options.

## Key Design Decisions

- Expression formatting uses the same recursive `extract()` pattern as the Python original. Each regex pattern is tried in priority order; the first match splits the line into three parts which are recursively formatted.
- Matrix/cell indent uses the *content-relative* column (excluding leading whitespace) to match the Python behavior where indent is computed from the non-whitespace content before the opening bracket.
- `indent.rs` returns an `IndentResult` with offset, formatted line, and flags (`skip_expression_fmt`, `is_ctrl_ignore`) so `lib.rs` knows which lines to expression-format.
- Trailing blank lines are stripped from output to match Python behavior.
