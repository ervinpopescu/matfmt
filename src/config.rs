use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, ValueEnum)]
pub enum IndentMode {
    AllFunctions,
    OnlyNested,
    Classic,
}

#[derive(Debug, Clone, Copy, PartialEq, ValueEnum)]
pub enum OperatorSpacing {
    AllOperators,
    ExcludePow,
    NoSpaces,
}

#[derive(Debug, Clone, Copy, PartialEq, ValueEnum)]
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
