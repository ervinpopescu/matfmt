pub mod config;
pub mod expression;
pub mod indent;
pub mod matrix;

pub use config::*;

use indent::IndentEngine;
use matrix::MatrixTracker;

lazy_static::lazy_static! {
    static ref P_BLANK: regex::Regex = regex::Regex::new(r"^\s*$").unwrap();
}

pub struct Formatter {
    config: FormatterConfig,
    indent: IndentEngine,
    matrix: MatrixTracker,
}

impl Formatter {
    pub fn new(config: FormatterConfig) -> Self {
        let indent = IndentEngine::new(&config);
        Formatter {
            config,
            indent,
            matrix: MatrixTracker::new(),
        }
    }

    pub fn format(&mut self, text: &str) -> String {
        let mut formatted_text = String::new();
        let mut blank = true;

        for line in text.lines() {
            // Skip blank lines (collapse multiple blanks)
            if P_BLANK.is_match(line) {
                if !blank {
                    blank = true;
                    formatted_text.push('\n');
                }
                continue;
            }

            // Check matrix/cell state before indentation
            let matrix_indent = self.matrix.update(line, &self.config);

            // Process indentation and control structure detection
            let result = self.indent.process_line(line, &self.config);

            // Build the final line
            let final_line = if let Some(mind) = matrix_indent {
                // Inside a multi-line matrix/cell: use matrix indent override
                let indent_str = " ".repeat(
                    (self.indent.ilvl * self.config.indent_width as i32 + mind).max(0) as usize,
                );
                let content = if result.skip_expression_fmt {
                    line.trim().to_string()
                } else {
                    expression::format_expression(line, &self.config)
                        .trim()
                        .to_string()
                };
                format!("{}{}", indent_str, content)
            } else if result.skip_expression_fmt || result.is_ctrl_ignore {
                // Comments, ignored lines, imports: indent engine already built the line
                result.line.clone()
            } else {
                // Normal line: the indent engine provides the keyword-formatted line
                // with correct indentation. For lines that have expression-formattable
                // content, we need to expression-format the body.
                // The indent engine has already handled keywords (if/for/end/function etc.)
                // and their indentation. For keyword lines, the result.line includes
                // the keyword + body. For plain lines, it's indent + trimmed content.
                // We need to expression-format the body part while keeping the indent
                // engine's indentation and keyword handling.
                let trimmed = line.trim();
                let expr_formatted = expression::format_expression(trimmed, &self.config);
                // Replace the trimmed content in the indent result with expression-formatted
                let indent_prefix = &result.line[..result.line.len() - result.line.trim_start().len()];
                format!("{}{}", indent_prefix, expr_formatted.trim())
            };

            // Add blank line before block (if separate_blocks enabled)
            if self.config.separate_blocks
                && result.offset > 0
                && !blank
                && self.indent.is_line_comment == 0
            {
                formatted_text.push('\n');
            }

            formatted_text.push_str(&final_line.trim_end());
            formatted_text.push('\n');
            blank = false;

            // Add blank line after block
            if self.config.separate_blocks && result.offset < 0 {
                formatted_text.push('\n');
                blank = true;
            }

            // Apply indent offset after the line
            self.indent.ilvl = (self.indent.ilvl + result.offset).max(0);
        }

        // Remove trailing blank lines (matching Python behavior)
        while formatted_text.ends_with("\n\n") {
            formatted_text.pop();
        }

        formatted_text
    }
}
