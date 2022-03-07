use data_conversion::services::convert::to_hex::convert_to_hex;
use crate::cli::parser::{ConvertCli, CommandsCli, ConversionType, CliParser};

pub fn run(cli: &CliParser) {
    match &cli.command {
        CommandsCli::Convert(convert_params) => convert(convert_params),
    }
    println!();
}

fn convert(params: &ConvertCli) {
    match &params.conversion_type {
        ConversionType::ToHex => to_hex_print(&params.input_data),
    }
}

fn to_hex_print(input_data: &str) {
    let hex_data = convert_to_hex(&input_data);
    print!("{}", hex_data);
}
