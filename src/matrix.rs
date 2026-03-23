use crate::config::{FormatterConfig, MatrixIndent};
use crate::expression;

pub struct MatrixTracker {
    bracket_depth: i32,
    brace_depth: i32,
    bracket_indent: i32,
    brace_indent: i32,
}

impl MatrixTracker {
    pub fn new() -> Self {
        MatrixTracker {
            bracket_depth: 0,
            brace_depth: 0,
            bracket_indent: 0,
            brace_indent: 0,
        }
    }

    fn cell_indent(
        line: &str,
        open_char: char,
        close_char: char,
        current_indent: i32,
        config: &FormatterConfig,
    ) -> (i32, i32) {
        let cleaned = expression::clean_strings_and_comments(line);
        let opened = cleaned.matches(open_char).count() as i32
            - cleaned.matches(close_char).count() as i32;

        let indent = if opened > 0 {
            if config.matrix_indent == MatrixIndent::Aligned {
                if let Some(pos) = cleaned.find(open_char) {
                    // Count content before the bracket, excluding leading whitespace
                    // This matches Python's behavior: regex group(2) captures non-ws content before bracket
                    let before = &cleaned[..pos];
                    let content_before = before.trim_start();
                    (content_before.chars().count() as i32) + 1
                } else {
                    config.indent_width as i32
                }
            } else {
                config.indent_width as i32
            }
        } else if opened < 0 {
            0
        } else {
            current_indent
        };

        (opened, indent)
    }

    /// Update matrix/cell tracking for this line.
    /// Returns Some(indent_override) if we're inside a multi-line matrix/cell,
    /// None if normal indentation should be used.
    ///
    /// Following the Python logic:
    /// - Save previous indent (tmp)
    /// - Update depth/indent
    /// - If depth changed OR was already inside (tmp > 0), return Some(tmp)
    /// - Opening line: tmp was 0 before open, so returns None (normal indent for opening line)
    /// - Continuation/closing line: tmp > 0, so returns Some(tmp)
    pub fn update(&mut self, line: &str, config: &FormatterConfig) -> Option<i32> {
        // Check brackets [] (Python: multilinematrix)
        let prev_bracket_indent = self.bracket_indent;
        let prev_bracket_depth = self.bracket_depth;
        let (bracket_opened, bracket_indent) =
            Self::cell_indent(line, '[', ']', self.bracket_indent, config);
        self.bracket_depth += bracket_opened;
        self.bracket_indent = bracket_indent;

        // If we were inside a bracket matrix or depth changed, use bracket indent
        if bracket_opened != 0 || prev_bracket_depth > 0 {
            return Some(prev_bracket_indent);
        }

        // Check braces {} (Python: cellarray)
        let prev_brace_indent = self.brace_indent;
        let prev_brace_depth = self.brace_depth;
        let (brace_opened, brace_indent) =
            Self::cell_indent(line, '{', '}', self.brace_indent, config);
        self.brace_depth += brace_opened;
        self.brace_indent = brace_indent;

        if brace_opened != 0 || prev_brace_depth > 0 {
            return Some(prev_brace_indent);
        }

        None
    }
}
