use lazy_static::lazy_static;
use regex::Regex;

use crate::config::{FormatterConfig, IndentMode};
use crate::expression;

lazy_static! {
    static ref LINECOMMENT: Regex = Regex::new(r"^\s*%.*$").unwrap();
    static ref BLOCKCOMMENT_OPEN: Regex = Regex::new(r"^\s*%\{\s*$").unwrap();
    static ref BLOCKCOMMENT_CLOSE: Regex = Regex::new(r"^\s*%\}\s*$").unwrap();
    static ref IGNORE_COMMAND: Regex = Regex::new(r".*formatter\s+ignore\s+(\d*).*$").unwrap();
    static ref FCNSTART: Regex = Regex::new(r"^\s*(function|classdef)\s*(.*)$").unwrap();
    static ref CTRLSTART: Regex = Regex::new(r"^\s*(if|while|for|parfor|try|methods|properties|events|arguments|enumeration|spmd)\s*(.*)$").unwrap();
    static ref CTRLSTART_2: Regex = Regex::new(r"^\s*(switch)\s*(.*)$").unwrap();
    static ref CTRLCONT: Regex = Regex::new(r"^\s*(elseif|else|case|otherwise|catch)\s*(.*)$").unwrap();
    static ref CTRLEND: Regex = Regex::new(r"^\s*((end|endfunction|endif|endwhile|endfor|endswitch);?)\s*(.*)$").unwrap();
    static ref CTRL_1LINE: Regex = Regex::new(r"^\s*(if|while|for|try)(\W\s*\S.*\W)((end|endif|endwhile|endfor);?)(\s+\S.*|\s*)$").unwrap();
    static ref CTRL_IGNORE: Regex = Regex::new(r"^\s*(import|clear|clearvars)(.*$)").unwrap();
    static ref ELLIPSIS: Regex = Regex::new(r".*\.\.\..*$").unwrap();
    static ref BLOCK_CLOSE: Regex = Regex::new(r"^\s*[\)\]\}].*$").unwrap();
}

pub struct IndentEngine {
    pub ilvl: i32,
    iwidth: i32,
    istep: Vec<i32>,
    fstep: Vec<i32>,
    indent_mode: IndentMode,
    pub is_block_comment: bool,
    pub is_line_comment: i32,
    ignore_lines: i32,
    longline: bool,
    continueline: bool,
}

/// Result from processing a single line
pub struct IndentResult {
    /// Indent offset to apply after this line
    pub offset: i32,
    /// The formatted line content (keyword-normalized, trimmed)
    pub line: String,
    /// Whether this line should skip expression formatting
    pub skip_expression_fmt: bool,
    /// Whether this line is a control-ignore (import/clear/clearvars)
    pub is_ctrl_ignore: bool,
}

impl IndentEngine {
    pub fn new(config: &FormatterConfig) -> Self {
        IndentEngine {
            ilvl: 0,
            iwidth: config.indent_width as i32,
            istep: Vec::new(),
            fstep: Vec::new(),
            indent_mode: config.indent_mode,
            is_block_comment: false,
            is_line_comment: 0,
            ignore_lines: 0,
            longline: false,
            continueline: false,
        }
    }

    pub fn indent(&self, addspaces: i32) -> String {
        let cont = if self.continueline { self.iwidth } else { 0 };
        " ".repeat(((self.ilvl) * self.iwidth + cont + addspaces).max(0) as usize)
    }

    pub fn process_line(&mut self, line: &str, _config: &FormatterConfig) -> IndentResult {
        let trimmed = line.trim();

        // Handle ignored lines
        if self.ignore_lines > 0 {
            self.ignore_lines -= 1;
            return IndentResult {
                offset: 0,
                line: self.indent(0) + trimmed,
                skip_expression_fmt: true,
                is_ctrl_ignore: false,
            };
        }

        // Track line comments
        if LINECOMMENT.is_match(line) {
            self.is_line_comment = 2;
        } else {
            self.is_line_comment = (self.is_line_comment - 1).max(0);
        }

        // Track block comments
        if BLOCKCOMMENT_OPEN.is_match(line) {
            self.is_block_comment = true;
        } else if BLOCKCOMMENT_CLOSE.is_match(line) {
            // Process this line as block comment, then turn off
            let result = IndentResult {
                offset: 0,
                line: line.trim_end().to_string(),
                skip_expression_fmt: true,
                is_ctrl_ignore: false,
            };
            self.is_block_comment = false;
            return result;
        }

        // Ellipsis / continuation tracking
        let stripped = expression::clean_strings_and_comments(line);
        let ellipsis_in_comment = self.is_line_comment == 2 || self.is_block_comment;
        if BLOCK_CLOSE.is_match(&stripped) || ellipsis_in_comment {
            self.continueline = false;
        } else {
            self.continueline = self.longline;
        }
        if ELLIPSIS.is_match(&stripped) && !ellipsis_in_comment {
            self.longline = true;
        } else {
            self.longline = false;
        }

        // Block comments: don't modify indentation
        if self.is_block_comment {
            return IndentResult {
                offset: 0,
                line: line.trim_end().to_string(),
                skip_expression_fmt: true,
                is_ctrl_ignore: false,
            };
        }

        // Line comments
        if self.is_line_comment == 2 {
            if let Some(caps) = IGNORE_COMMAND.captures(trimmed) {
                if let Some(num_str) = caps.get(1) {
                    let s = num_str.as_str();
                    if !s.is_empty() {
                        if let Ok(num) = s.parse::<i32>() {
                            self.ignore_lines = if num > 1 { num } else { 1 };
                        }
                    } else {
                        self.ignore_lines = 1;
                    }
                }
            }
            return IndentResult {
                offset: 0,
                line: self.indent(0) + trimmed,
                skip_expression_fmt: true,
                is_ctrl_ignore: false,
            };
        }

        // Control-ignore (import, clear, clearvars)
        if CTRL_IGNORE.is_match(line) {
            return IndentResult {
                offset: 0,
                line: self.indent(0) + trimmed,
                skip_expression_fmt: true,
                is_ctrl_ignore: true,
            };
        }

        // One-line control structure (if ... end)
        if let Some(caps) = CTRL_1LINE.captures(line) {
            let keyword = &caps[1];
            let body = &caps[2];
            let end_kw = &caps[3];
            let after = caps.get(5).map_or("", |m| m.as_str()).trim();
            let formatted = if after.is_empty() {
                format!("{}{} {} {}", self.indent(0), keyword, body.trim(), end_kw)
            } else {
                format!("{}{} {} {} {}", self.indent(0), keyword, body.trim(), end_kw, after)
            };
            return IndentResult {
                offset: 0,
                line: formatted,
                skip_expression_fmt: false,
                is_ctrl_ignore: false,
            };
        }

        // Function/classdef start
        if let Some(caps) = FCNSTART.captures(line) {
            let offset = match self.indent_mode {
                IndentMode::AllFunctions => 1,
                IndentMode::Classic => 0,
                IndentMode::OnlyNested => {
                    if !self.fstep.is_empty() { 1 } else { 0 }
                }
            };
            self.fstep.push(1);
            let content = format!("{}{} {}", self.indent(0), &caps[1], caps[2].trim());
            return IndentResult {
                offset,
                line: content,
                skip_expression_fmt: false,
                is_ctrl_ignore: false,
            };
        }

        // Control start (if, while, for, etc.)
        if let Some(caps) = CTRLSTART.captures(line) {
            self.istep.push(1);
            let content = format!("{}{} {}", self.indent(0), &caps[1], caps[2].trim());
            return IndentResult {
                offset: 1,
                line: content,
                skip_expression_fmt: false,
                is_ctrl_ignore: false,
            };
        }

        // Switch
        if let Some(caps) = CTRLSTART_2.captures(line) {
            self.istep.push(2);
            let content = format!("{}{} {}", self.indent(0), &caps[1], caps[2].trim());
            return IndentResult {
                offset: 2,
                line: content,
                skip_expression_fmt: false,
                is_ctrl_ignore: false,
            };
        }

        // Control continuation (elseif, else, case, otherwise, catch)
        if let Some(caps) = CTRLCONT.captures(line) {
            let content = format!("{}{} {}", self.indent(-self.iwidth), &caps[1], caps[2].trim());
            return IndentResult {
                offset: 0,
                line: content,
                skip_expression_fmt: false,
                is_ctrl_ignore: false,
            };
        }

        // Control end
        if let Some(caps) = CTRLEND.captures(line) {
            let step = if !self.istep.is_empty() {
                self.istep.pop().unwrap()
            } else if !self.fstep.is_empty() {
                self.fstep.pop().unwrap()
            } else {
                0
            };
            let end_keyword = &caps[1];
            let after = caps.get(3).map_or("", |m| m.as_str()).trim();
            let content = if after.is_empty() {
                format!("{}{}", self.indent(-step * self.iwidth), end_keyword)
            } else {
                format!("{}{} {}", self.indent(-step * self.iwidth), end_keyword, after)
            };
            return IndentResult {
                offset: -step,
                line: content,
                skip_expression_fmt: false,
                is_ctrl_ignore: false,
            };
        }

        // Default: just indent
        IndentResult {
            offset: 0,
            line: self.indent(0) + trimmed,
            skip_expression_fmt: false,
            is_ctrl_ignore: false,
        }
    }
}
