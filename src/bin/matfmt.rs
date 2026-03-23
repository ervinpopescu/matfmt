use clap::Parser;
use matfmt::{Formatter, FormatterConfig, IndentMode, MatrixIndent, OperatorSpacing};
use std::fs;
use std::io::{self, Read};

#[derive(Parser)]
#[command(name = "matfmt", about = "MATLAB code formatter")]
struct Cli {
    /// Input file (use - for stdin)
    file: String,

    /// Spaces per indent level
    #[arg(long, default_value_t = 4)]
    indent_width: u32,

    /// Function indent mode
    #[arg(long, value_enum, default_value_t = IndentMode::AllFunctions)]
    indent_mode: IndentMode,

    /// Operator spacing mode
    #[arg(long, value_enum, default_value_t = OperatorSpacing::ExcludePow)]
    operator_spacing: OperatorSpacing,

    /// Matrix continuation indent mode
    #[arg(long, value_enum, default_value_t = MatrixIndent::Aligned)]
    matrix_indent: MatrixIndent,

    /// Don't insert blank lines around control blocks
    #[arg(long)]
    no_separate_blocks: bool,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let input = if cli.file == "-" {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        buf
    } else {
        fs::read_to_string(&cli.file)?
    };

    let config = FormatterConfig {
        indent_width: cli.indent_width,
        separate_blocks: !cli.no_separate_blocks,
        indent_mode: cli.indent_mode,
        operator_spacing: cli.operator_spacing,
        matrix_indent: cli.matrix_indent,
    };

    let mut formatter = Formatter::new(config);
    print!("{}", formatter.format(&input));

    Ok(())
}
