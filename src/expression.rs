use lazy_static::lazy_static;
use regex::Regex;

use crate::config::{FormatterConfig, OperatorSpacing};

lazy_static! {
    static ref P_STRING: Regex = Regex::new(r"^(.*?[\(\[\{,;=\+\-\*/\|\&\s]|^)\s*('([^']|'')+')([\)\}\]\+\-\*/=\|\&,;].*|\s+.*|$)").unwrap();
    static ref P_STRING_DQ: Regex = Regex::new(r#"^(.*?[\(\[\{,;=\+\-\*/\|\&\s]|^)\s*("([^"])*")([\)\}\]\+\-\*/=\|\&,;].*|\s+.*|$)"#).unwrap();
    static ref P_COMMENT: Regex = Regex::new(r"^(.*\S|^)\s*(%.*)\s*$").unwrap();
    static ref P_BLANK: Regex = Regex::new(r"^\s+$").unwrap();
    static ref P_NUM_SC: Regex = Regex::new(r"^(.*?\W|^)\s*(\d+\.?\d*)([eE][+-]?)(\d+)(.*)$").unwrap();
    static ref P_NUM_R: Regex = Regex::new(r"^(.*?\W|^)\s*(\d+)\s*(/)\s*(\d+)(.*)$").unwrap();
    static ref P_INCR: Regex = Regex::new(r"^(.*?\S|^)\s*(\+|-)\s*(\+|-)\s*([\)\]\},;].*|$)").unwrap();
    static ref P_SIGN: Regex = Regex::new(r"^(.*?[\(\[\{,;:=\*/\s]|^)\s*(\+|-)(\w.*)$").unwrap();
    static ref P_COLON: Regex = Regex::new(r"^(.*?\S|^)\s*(:)\s*(\S.*|$)$").unwrap();
    static ref P_ELLIPSIS: Regex = Regex::new(r"^(.*?\S|^)\s*(\.\.\.)\s*(\S.*|$)$").unwrap();
    static ref P_OP_DOT: Regex = Regex::new(r"^(.*?\S|^)\s*(\.)\s*(\+|-|\*|/|\^)\s*(=)\s*(\S.*|$)$").unwrap();
    static ref P_POW_DOT: Regex = Regex::new(r"^(.*?\S|^)\s*(\.)\s*(\^)\s*(\S.*|$)$").unwrap();
    static ref P_POW: Regex = Regex::new(r"^(.*?\S|^)\s*(\^)\s*(\S.*|$)$").unwrap();
    static ref P_OP_COMB: Regex = Regex::new(r"^(.*?\S|^)\s*(\.|\+|-|\*|\\|/|=|<|>|\||\&|!|~|\^)\s*(<|>|=|\+|-|\*|/|\&|\|)\s*(\S.*|$)$").unwrap();
    static ref P_NOT: Regex = Regex::new(r"^(.*?\S|^)\s*(!|~)\s*(\S.*)$").unwrap();
    static ref P_OP: Regex = Regex::new(r"^(.*?\S|^)\s*(\+|-|\*|\\|/|=|!|~|<|>|\||\&)\s*(\S.*|$)$").unwrap();
    static ref P_FUNC: Regex = Regex::new(r"^(.*?\w)(\()\s*(\S.*|$)$").unwrap();
    static ref P_OPEN: Regex = Regex::new(r"^(.*?)(\(|\[|\{)\s*(\S.*|$)$").unwrap();
    static ref P_CLOSE: Regex = Regex::new(r"^(.*?\S|^)\s*(\)|\]|\})(.*|$)$").unwrap();
    static ref P_COMMA: Regex = Regex::new(r"^(.*?\S|^)\s*(,|;)\s*(\S.*|$)$").unwrap();
    static ref P_MULTIWS: Regex = Regex::new(r"^(.*?\S|^)(\s{2,})(\S.*|$)$").unwrap();
}

struct ExtractState {
    is_comment: bool,
}

/// Extract a string or comment from the line, returning (before, token, after)
fn extract_string_comment(part: &str, state: &mut ExtractState) -> Option<(String, String, String)> {
    // Single-quoted string
    let m1 = P_STRING.captures(part);
    // Double-quoted string
    let m2 = P_STRING_DQ.captures(part);

    // Choose the longer string match to avoid extracting subexpressions
    let m = match (&m1, &m2) {
        (Some(c1), Some(c2)) => {
            if c1.get(2).unwrap().as_str().len() < c2.get(2).unwrap().as_str().len() {
                Some(c2)
            } else {
                Some(c1)
            }
        }
        (Some(c1), None) => Some(c1),
        (None, Some(c2)) => Some(c2),
        (None, None) => None,
    };

    if let Some(caps) = m {
        return Some((
            caps[1].to_string(),
            caps[2].to_string(),
            caps[4].to_string(),
        ));
    }

    // Comment
    if let Some(caps) = P_COMMENT.captures(part) {
        state.is_comment = true;
        return Some((
            caps[1].to_string() + " ",
            caps[2].to_string(),
            String::new(),
        ));
    }

    None
}

/// Extract a pattern from the line, returning (before, token, after)
fn extract(part: &str, config: &FormatterConfig, state: &mut ExtractState) -> Option<(String, String, String)> {
    // Whitespace only
    if P_BLANK.is_match(part) {
        return Some((String::new(), " ".to_string(), String::new()));
    }

    // String or comment
    if let Some(result) = extract_string_comment(part, state) {
        return Some(result);
    }

    // Scientific notation (e.g. 5.6E-3)
    if let Some(caps) = P_NUM_SC.captures(part) {
        return Some((
            format!("{}{}", &caps[1], &caps[2]),
            caps[3].to_string(),
            format!("{}{}", &caps[4], &caps[5]),
        ));
    }

    // Rational number (e.g. 1/4)
    if let Some(caps) = P_NUM_R.captures(part) {
        return Some((
            format!("{}{}", &caps[1], &caps[2]),
            caps[3].to_string(),
            format!("{}{}", &caps[4], &caps[5]),
        ));
    }

    // Increment (++ or --)
    if let Some(caps) = P_INCR.captures(part) {
        return Some((
            caps[1].to_string(),
            format!("{}{}", &caps[2], &caps[3]),
            caps[4].to_string(),
        ));
    }

    // Unary sign
    if let Some(caps) = P_SIGN.captures(part) {
        return Some((
            caps[1].to_string(),
            caps[2].to_string(),
            caps[3].to_string(),
        ));
    }

    // Colon
    if let Some(caps) = P_COLON.captures(part) {
        return Some((
            caps[1].to_string(),
            caps[2].to_string(),
            caps[3].to_string(),
        ));
    }

    // Dot-operator-assignment (e.g. .+=)
    if let Some(caps) = P_OP_DOT.captures(part) {
        let sep = if config.operator_spacing != OperatorSpacing::NoSpaces { " " } else { "" };
        return Some((
            format!("{}{}", &caps[1], sep),
            format!("{}{}{}", &caps[2], &caps[3], &caps[4]),
            format!("{}{}", sep, &caps[5]),
        ));
    }

    // Dot-power (.^)
    if let Some(caps) = P_POW_DOT.captures(part) {
        let sep = if config.operator_spacing == OperatorSpacing::AllOperators { " " } else { "" };
        return Some((
            format!("{}{}", &caps[1], sep),
            format!("{}{}", &caps[2], &caps[3]),
            format!("{}{}", sep, &caps[4]),
        ));
    }

    // Power (^)
    if let Some(caps) = P_POW.captures(part) {
        let sep = if config.operator_spacing == OperatorSpacing::AllOperators { " " } else { "" };
        return Some((
            format!("{}{}", &caps[1], sep),
            caps[2].to_string(),
            format!("{}{}", sep, &caps[3]),
        ));
    }

    // Combined operator (e.g. +=, .+, <=, &&, ||)
    if let Some(caps) = P_OP_COMB.captures(part) {
        let sep = if config.operator_spacing != OperatorSpacing::NoSpaces { " " } else { "" };
        return Some((
            format!("{}{}", &caps[1], sep),
            format!("{}{}", &caps[2], &caps[3]),
            format!("{}{}", sep, &caps[4]),
        ));
    }

    // Not (~ or !)
    if let Some(caps) = P_NOT.captures(part) {
        return Some((
            format!("{} ", &caps[1]),
            caps[2].to_string(),
            caps[3].to_string(),
        ));
    }

    // Single operator
    if let Some(caps) = P_OP.captures(part) {
        let sep = if config.operator_spacing != OperatorSpacing::NoSpaces { " " } else { "" };
        return Some((
            format!("{}{}", &caps[1], sep),
            caps[2].to_string(),
            format!("{}{}", sep, &caps[3]),
        ));
    }

    // Function call
    if let Some(caps) = P_FUNC.captures(part) {
        return Some((
            caps[1].to_string(),
            caps[2].to_string(),
            caps[3].to_string(),
        ));
    }

    // Open paren/bracket/brace
    if let Some(caps) = P_OPEN.captures(part) {
        return Some((
            caps[1].to_string(),
            caps[2].to_string(),
            caps[3].to_string(),
        ));
    }

    // Close paren/bracket/brace
    if let Some(caps) = P_CLOSE.captures(part) {
        return Some((
            caps[1].to_string(),
            caps[2].to_string(),
            caps[3].to_string(),
        ));
    }

    // Comma/semicolon
    if let Some(caps) = P_COMMA.captures(part) {
        return Some((
            caps[1].to_string(),
            caps[2].to_string(),
            format!(" {}", &caps[3]),
        ));
    }

    // Ellipsis
    if let Some(caps) = P_ELLIPSIS.captures(part) {
        return Some((
            format!("{} ", &caps[1]),
            caps[2].to_string(),
            format!(" {}", &caps[3]),
        ));
    }

    // Multiple whitespace
    if let Some(caps) = P_MULTIWS.captures(part) {
        return Some((
            caps[1].to_string(),
            " ".to_string(),
            caps[3].to_string(),
        ));
    }

    None
}

/// Recursively format expression by extracting and formatting sub-expressions
pub fn format_expression(part: &str, config: &FormatterConfig) -> String {
    let mut state = ExtractState { is_comment: false };
    format_recursive(part, config, &mut state)
}

fn format_recursive(part: &str, config: &FormatterConfig, state: &mut ExtractState) -> String {
    if let Some((before, token, after)) = extract(part, config, state) {
        format!(
            "{}{}{}",
            format_recursive(&before, config, state),
            token,
            format_recursive(&after, config, state)
        )
    } else {
        part.to_string()
    }
}

/// Strip strings and comments from a line, replacing them with spaces.
/// Used by matrix tracking to count brackets without being confused by strings/comments.
pub fn clean_strings_and_comments(line: &str) -> String {
    let mut state = ExtractState { is_comment: false };
    clean_recursive(line, &mut state)
}

fn clean_recursive(part: &str, state: &mut ExtractState) -> String {
    if let Some((before, _token, after)) = extract_string_comment(part, state) {
        format!(
            "{} {}",
            clean_recursive(&before, state),
            clean_recursive(&after, state)
        )
    } else {
        part.to_string()
    }
}
