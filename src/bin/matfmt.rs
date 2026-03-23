use clap::Parser;
use matfmt::{FileConfig, Formatter, IndentMode, MatrixIndent, OperatorSpacing};
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "matfmt", about = "MATLAB code formatter")]
struct Cli {
    /// Input file (use - for stdin)
    file: String,

    /// Config file path (default: matfmt.toml in current dir)
    #[arg(long)]
    config: Option<PathBuf>,

    /// Spaces per indent level
    #[arg(long)]
    indent_width: Option<u32>,

    /// Function indent mode
    #[arg(long, value_enum)]
    indent_mode: Option<IndentMode>,

    /// Operator spacing mode
    #[arg(long, value_enum)]
    operator_spacing: Option<OperatorSpacing>,

    /// Matrix continuation indent mode
    #[arg(long, value_enum)]
    matrix_indent: Option<MatrixIndent>,

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

    // Load config: defaults <- config file <- CLI flags
    let config_path = cli.config.unwrap_or_else(|| PathBuf::from("matfmt.toml"));
    let file_config = FileConfig::load(&config_path).unwrap_or_default();
    let mut config = file_config.into_config();

    // CLI flags override file config
    if let Some(v) = cli.indent_width {
        config.indent_width = v;
    }
    if let Some(v) = cli.indent_mode {
        config.indent_mode = v;
    }
    if let Some(v) = cli.operator_spacing {
        config.operator_spacing = v;
    }
    if let Some(v) = cli.matrix_indent {
        config.matrix_indent = v;
    }
    if cli.no_separate_blocks {
        config.separate_blocks = false;
    }

    let mut formatter = Formatter::new(config);
    print!("{}", formatter.format(&input));

    Ok(())
}
