use clap::{Parser, Subcommand, ArgEnum};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct CliParser {
    #[clap(subcommand)]
    pub command: CommandsCli,
}

#[derive(Subcommand)]
pub enum CommandsCli {
    /// Converts input data to TODO
    Convert(ConvertCli),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum ConversionType {
    ToHex,
    HexToString,
}

#[derive(Parser)]
pub struct ConvertCli {
    /// Data to be converted.
    pub input_data: String,

    /// Conversion type
    #[clap(short, long, arg_enum)]
    pub conversion_type: ConversionType
}
