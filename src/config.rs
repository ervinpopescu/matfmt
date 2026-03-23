use clap::ValueEnum;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, ValueEnum, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IndentMode {
    AllFunctions,
    OnlyNested,
    Classic,
}

#[derive(Debug, Clone, Copy, PartialEq, ValueEnum, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorSpacing {
    AllOperators,
    ExcludePow,
    NoSpaces,
}

#[derive(Debug, Clone, Copy, PartialEq, ValueEnum, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatrixIndent {
    Aligned,
    Simple,
}

#[derive(Debug, Clone)]
pub struct FormatterConfig {
    pub indent_width: u32,
    pub separate_blocks: bool,
    pub indent_mode: IndentMode,
    pub operator_spacing: OperatorSpacing,
    pub matrix_indent: MatrixIndent,
}

impl Default for FormatterConfig {
    fn default() -> Self {
        Self {
            indent_width: 4,
            separate_blocks: true,
            indent_mode: IndentMode::AllFunctions,
            operator_spacing: OperatorSpacing::ExcludePow,
            matrix_indent: MatrixIndent::Aligned,
        }
    }
}

/// Partial config loaded from TOML. All fields optional so they only
/// override defaults when present.
#[derive(Debug, Deserialize, Default)]
pub struct FileConfig {
    pub indent_width: Option<u32>,
    pub separate_blocks: Option<bool>,
    pub indent_mode: Option<IndentMode>,
    pub operator_spacing: Option<OperatorSpacing>,
    pub matrix_indent: Option<MatrixIndent>,
}

impl FileConfig {
    pub fn load(path: &std::path::Path) -> Option<Self> {
        let content = std::fs::read_to_string(path).ok()?;
        toml::from_str(&content).ok()
    }

    /// Apply file config on top of defaults, producing a full config.
    pub fn into_config(self) -> FormatterConfig {
        let defaults = FormatterConfig::default();
        FormatterConfig {
            indent_width: self.indent_width.unwrap_or(defaults.indent_width),
            separate_blocks: self.separate_blocks.unwrap_or(defaults.separate_blocks),
            indent_mode: self.indent_mode.unwrap_or(defaults.indent_mode),
            operator_spacing: self.operator_spacing.unwrap_or(defaults.operator_spacing),
            matrix_indent: self.matrix_indent.unwrap_or(defaults.matrix_indent),
        }
    }
}
