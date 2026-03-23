# matfmt

MATLAB code formatter in Rust. Formats `.m` files: fixes indentation, normalizes operator spacing, aligns multi-line matrices, and handles line continuations (`...`).

Port of [matlab-formatter-vscode](https://github.com/AEditorFormatter/matlab-formatter-vscode).

## Install

```sh
cargo install --path .
```

## Quick start

```sh
matfmt input.m              # format to stdout
cat input.m | matfmt -      # stdin works too
matfmt input.m --indent-width 2
```

Before:
```matlab
function result=compute(a,b,c)
if a>0
x=a+b*c;
M=[1 2 3;
4 5 6];
end
end
```

After:
```matlab
function result = compute(a, b, c)

    if a > 0
        x = a + b * c;
        M = [1 2 3;
             4 5 6];
    end

end
```

## Options

```
--indent-width <N>           spaces per indent level (default: 4)
--indent-mode <MODE>         all-functions | only-nested | classic
--operator-spacing <MODE>    all-operators | exclude-pow | no-spaces
--matrix-indent <MODE>       aligned | simple
--no-separate-blocks         skip blank lines around control blocks
```

`--indent-mode` controls function body indentation: `all-functions` indents everything, `only-nested` skips the outermost function, `classic` doesn't indent function bodies at all.

`--operator-spacing` controls whitespace around operators. `exclude-pow` (default) leaves `^` and `.^` tight, `all-operators` spaces everything, `no-spaces` removes all operator whitespace.

`--matrix-indent` controls continuation lines in multi-line `[]` and `{}`: `aligned` lines up with the opening bracket, `simple` just uses the indent width.

## As a library

```rust
use matfmt::{Formatter, FormatterConfig};

let mut f = Formatter::new(FormatterConfig::default());
assert_eq!(f.format("a=1+2;"), "a = 1 + 2;\n");
```
