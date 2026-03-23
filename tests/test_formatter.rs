use std::path::Path;

use matfmt::{Formatter, FormatterConfig, IndentMode, OperatorSpacing, MatrixIndent};

fn fixture(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name);
    std::fs::read_to_string(path).unwrap()
}

fn fmt(input: &str) -> String {
    let mut f = Formatter::new(FormatterConfig::default());
    f.format(input)
}

fn fmt_cfg(input: &str, config: FormatterConfig) -> String {
    let mut f = Formatter::new(config);
    f.format(input)
}

fn no_sep() -> FormatterConfig {
    FormatterConfig { separate_blocks: false, ..Default::default() }
}

// === Basic indentation ===

#[test]
fn test_simple_line() {
    assert_eq!(fmt("a = 1"), "a = 1\n");
}

#[test]
fn test_function_indent() {
    let input = "function y = foo(x)\ny = x + 1;\nend";
    let expected = "function y = foo(x)\n    y = x + 1;\nend\n";
    assert_eq!(fmt_cfg(input, no_sep()), expected);
}

#[test]
fn test_if_else_indent() {
    let input = "if true\na = 1;\nelse\na = 2;\nend";
    let expected = "if true\n    a = 1;\nelse\n    a = 2;\nend\n";
    assert_eq!(fmt_cfg(input, no_sep()), expected);
}

#[test]
fn test_nested_indent() {
    let input = "function foo()\nfor i = 1:10\nif true\na = 1;\nend\nend\nend";
    let expected = "function foo()\n    for i = 1:10\n        if true\n            a = 1;\n        end\n    end\nend\n";
    assert_eq!(fmt_cfg(input, no_sep()), expected);
}

#[test]
fn test_switch_indent() {
    let input = "switch x\ncase 1\na = 1;\ncase 2\na = 2;\notherwise\na = 3;\nend";
    let expected = "switch x\n    case 1\n        a = 1;\n    case 2\n        a = 2;\n    otherwise\n        a = 3;\nend\n";
    assert_eq!(fmt_cfg(input, no_sep()), expected);
}

#[test]
fn test_block_comment() {
    let input = "%{\n   block comment\n%}\na = 1;";
    let expected = "%{\n   block comment\n%}\na = 1;\n";
    assert_eq!(fmt_cfg(input, no_sep()), expected);
}

#[test]
fn test_ignore_directive() {
    let input = "% formatter ignore 1\nthis=a   *very*  ill";
    let expected = "% formatter ignore 1\nthis=a   *very*  ill\n";
    assert_eq!(fmt_cfg(input, no_sep()), expected);
}

#[test]
fn test_classic_indent_mode() {
    let input = "function foo()\na = 1;\nend";
    let expected = "function foo()\na = 1;\nend\n";
    let cfg = FormatterConfig {
        separate_blocks: false,
        indent_mode: IndentMode::Classic,
        ..Default::default()
    };
    assert_eq!(fmt_cfg(input, cfg), expected);
}

#[test]
fn test_only_nested_indent_mode() {
    let input = "function foo()\na = 1;\nfunction bar()\nb = 2;\nend\nend";
    let expected = "function foo()\na = 1;\n\nfunction bar()\n    b = 2;\nend\n\nend\n";
    let cfg = FormatterConfig {
        indent_mode: IndentMode::OnlyNested,
        ..Default::default()
    };
    assert_eq!(fmt_cfg(input, cfg), expected);
}

#[test]
fn test_one_line_control() {
    let input = "if true; x = 1; end";
    let expected = "if true; x = 1; end\n";
    assert_eq!(fmt_cfg(input, no_sep()), expected);
}

#[test]
fn test_ctrl_ignore() {
    let input = "function foo()\nimport util.*\nclear\na = 1;\nend";
    let expected = "function foo()\n    import util.*\n    clear\n    a = 1;\nend\n";
    assert_eq!(fmt_cfg(input, no_sep()), expected);
}

#[test]
fn test_separate_blocks() {
    let input = "a = 1;\nif true\nb = 2;\nend\nc = 3;";
    let expected = "a = 1;\n\nif true\n    b = 2;\nend\n\nc = 3;\n";
    assert_eq!(fmt(input), expected);
}

#[test]
fn test_blank_line_collapse() {
    let input = "a = 1;\n\n\n\nb = 2;";
    let expected = "a = 1;\n\nb = 2;\n";
    assert_eq!(fmt(input), expected);
}

// === Expression formatting ===

#[test]
fn test_operator_spacing() {
    assert_eq!(fmt("a=1+2;"), "a = 1 + 2;\n");
}

#[test]
fn test_comma_spacing() {
    assert_eq!(fmt("b = [1,2,3];"), "b = [1, 2, 3];\n");
}

#[test]
fn test_string_preservation() {
    assert_eq!(fmt("a = 'hello world';"), "a = 'hello world';\n");
}

#[test]
fn test_comment_preservation() {
    assert_eq!(fmt("a = 1; % this is a comment"), "a = 1; % this is a comment\n");
}

#[test]
fn test_scientific_notation() {
    assert_eq!(fmt("r = 42/0.8e15"), "r = 42/0.8e15\n");
}

#[test]
fn test_colon_no_spaces() {
    assert_eq!(fmt_cfg("for i = 1:10", no_sep()), "for i = 1:10\n");
}

#[test]
fn test_unary_minus() {
    assert_eq!(fmt("neg = -r"), "neg = -r\n");
}

#[test]
fn test_dot_operator() {
    assert_eq!(fmt("N = norm(a .* b - c)"), "N = norm(a .* b - c)\n");
}

#[test]
fn test_increment() {
    assert_eq!(fmt("k++"), "k++\n");
}

#[test]
fn test_multiple_whitespace_collapse() {
    assert_eq!(fmt("foo = -N   *   a(3)   /   N"), "foo = -N * a(3) / N\n");
}

#[test]
fn test_no_spaces_mode() {
    let cfg = FormatterConfig {
        operator_spacing: OperatorSpacing::NoSpaces,
        ..Default::default()
    };
    assert_eq!(fmt_cfg("a = 1 + 2;", cfg), "a=1+2;\n");
}

#[test]
fn test_double_quote_string() {
    assert_eq!(fmt("s = \"hello world\";"), "s = \"hello world\";\n");
}

#[test]
fn test_power_exclude_pow_mode() {
    assert_eq!(fmt("y = x ^ 2;"), "y = x^2;\n");
}

#[test]
fn test_power_all_operators_mode() {
    let cfg = FormatterConfig {
        operator_spacing: OperatorSpacing::AllOperators,
        ..Default::default()
    };
    assert_eq!(fmt_cfg("y = x^2;", cfg), "y = x ^ 2;\n");
}

#[test]
fn test_combined_operator() {
    assert_eq!(fmt("d += 4.7e11"), "d += 4.7e11\n");
}

// === Matrix/cell arrays ===

#[test]
fn test_matrix_aligned_indent() {
    let input = "M = [1 2 3;\n     4 5 6;\n     7 8 9]";
    let expected = "M = [1 2 3;\n     4 5 6;\n     7 8 9]\n";
    assert_eq!(fmt(input), expected);
}

#[test]
fn test_matrix_simple_indent() {
    let input = "M = [1 2 3;\n4 5 6;\n7 8 9]";
    let expected = "M = [1 2 3;\n    4 5 6;\n    7 8 9]\n";
    let cfg = FormatterConfig {
        matrix_indent: MatrixIndent::Simple,
        ..Default::default()
    };
    assert_eq!(fmt_cfg(input, cfg), expected);
}

#[test]
fn test_cell_array_aligned() {
    let input = "c = {1\n     [2 3]'\n     {1 M 'three'}};";
    let expected = "c = {1\n     [2 3]'\n     {1 M 'three'}};\n";
    assert_eq!(fmt(input), expected);
}

#[test]
fn test_single_line_matrix() {
    assert_eq!(fmt("b = [1, 2, 3];"), "b = [1, 2, 3];\n");
}

// === Ellipsis ===

#[test]
fn test_ellipsis_continuation() {
    let input = "t = a * k ...\n    +b .* k^2 ... % comment\n    +c * k^3";
    let expected = "t = a * k ...\n    +b .* k^2 ... % comment\n    +c * k^3\n";
    assert_eq!(fmt_cfg(input, no_sep()), expected);
}

#[test]
fn test_ellipsis_in_function_call() {
    let input = "tmp = sprintf( ...\n    'Test strings: %s %s %s', ...\n    \"Apple\", ... Apple\n    \"Banana\", ... Banana\n    \"Pear\" ... Pear\n);";
    let expected = "tmp = sprintf( ...\n    'Test strings: %s %s %s', ...\n    \"Apple\", ... Apple\n    \"Banana\", ... Banana\n    \"Pear\" ... Pear\n);\n";
    assert_eq!(fmt_cfg(input, no_sep()), expected);
}

// === Full file integration tests ===

#[test]
fn test_full_file_simple() {
    let input = "\
% This is a test file for the matlab-formatter.

function [output] = my_function(input1, input2)
    % This is a comment.
    a = 1+2;
    b = [1,2,3];
    c = { 'a' , 'b' , 'c' };

    if a > 0
        disp('positive');
    else
        disp('non-positive');
    end

    for i = 1:10
        disp(i);
    end
end";

    let expected = "\
% This is a test file for the matlab-formatter.

function [output] = my_function(input1, input2)
    % This is a comment.
    a = 1 + 2;
    b = [1, 2, 3];
    c = {'a', 'b', 'c'};

    if a > 0
        disp('positive');
    else
        disp('non-positive');
    end

    for i = 1:10
        disp(i);
    end

end
";

    assert_eq!(fmt(input), expected);
}

#[test]
fn test_full_file_post_formatter() {
    let input = fixture("post_formatter.m");
    let expected = "\
clear

%{
   block comment
%}
for (i = 1:5)

    if (1 == 1)
        i = i + 1
    end

end

tmp = sprintf( ...
    'Test strings: %s %s %s', ...
    \"Apple\", ... Apple
    \"Banana\", ... Banana
    \"Pear\" ... Pear
);
";

    assert_eq!(fmt(&input), expected);
}

// === File-based fixture tests ===

#[test]
fn test_fixture_classdef() {
    let input = fixture("classdef.m");
    let expected = "\
classdef MyClass

    properties
        x
        y
    end

    methods

        function obj = MyClass(a, b)
            obj.x = a;
            obj.y = b;
        end

        function r = add(obj)
            r = obj.x + obj.y;
        end

    end

end
";
    assert_eq!(fmt(&input), expected);
}

#[test]
fn test_fixture_comments_and_ignore() {
    let input = fixture("comments_and_ignore.m");
    let expected = "\
% Top-level comment
function foo()
    % line comment
    a = 1;

%{
  This is a block comment.
  It should not be reformatted.
    Weird    spacing   preserved.
%}

    % formatter ignore 2
    x=1+  2;
    y=3*   4;

    z = 5 + 6;
end
";
    assert_eq!(fmt(&input), expected);
}

#[test]
fn test_fixture_ellipsis() {
    let input = fixture("ellipsis.m");
    let expected = "\
function result = longcall(a, b, c)
    result = very_long_function_name(a, ...
        b, ...
        c);

    x = a + ...
        b + ...
        c;

    y = sprintf('%s %s %s', ...
        \"hello\", ... first
        \"world\", ... second
    \"!\"); ... third
    end
";
    assert_eq!(fmt(&input), expected);
}

#[test]
fn test_fixture_matrix_ops() {
    let input = fixture("matrix_ops.m");
    let expected = "\
function result = matops(A, B)
    % Matrix operations demo
    C = [1 2 3;
         4 5 6;
         7 8 9];

    D = {A
         B
         C};

    result = A * B + C .* D{3};

    M = [1 0 0;
         0 1 0;
         0 0 1];

    v = [1, 2, 3];
end
";
    assert_eq!(fmt(&input), expected);
}

#[test]
fn test_fixture_nested_functions() {
    let input = fixture("nested_functions.m");
    let expected = "\
function result = outer(x)
    result = inner(x) + 1;

    function y = inner(x)
        y = x * 2;
    end

end
";
    assert_eq!(fmt(&input), expected);
}

#[test]
fn test_fixture_operators() {
    let input = fixture("operators.m");
    let expected = "\
a = 1 + 2;
b = 3 - 4;
c = 5 * 6;
d = 7/8;
e = a == b;
f = a ~= b;
g = a >= b;
h = a <= b;
i = a && b;
j = a || b;
k = a.^2;
m = a^2;
n += 1;
p .+= 2;
q = -x;
r = +y;
s = ~flag;
t = !flag;
u = 'hello';
v = \"world\";
w = 1:10;
x = 1:2:10;
y = foo(a, b, c);
z = [1, 2; 3, 4];
";
    assert_eq!(fmt(&input), expected);
}

#[test]
fn test_fixture_switch_case() {
    let input = fixture("switch_case.m");
    let expected = "\
function grade = getGrade(score)

    switch score
        case 100
            grade = 'A+';
        case {90, 91, 92, 93, 94, 95, 96, 97, 98, 99}
            grade = 'A';
        otherwise
            grade = 'F';
    end

end
";
    assert_eq!(fmt(&input), expected);
}

#[test]
fn test_fixture_try_catch() {
    let input = fixture("try_catch.m");
    let expected = "\
function safe_divide(a, b)

    try
        result = a / b;
        disp(result);
    catch e
        fprintf('Error: %s\\n', e.message);
    end

end
";
    assert_eq!(fmt(&input), expected);
}
