use data_conversion::services::convert::{from_hex::FromHex, to_hex::ToHex};
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
        ConversionType::HexToString => hex_to_string_print(&params.input_data),
    }
}

fn to_hex_print(input_data: &str) {
    let hex_data = input_data.to_hex();
    print!("{}", hex_data);
}

fn hex_to_string_print(input_data: &str) {
    let result = input_data.hex_to_string();
    match result {
        Ok(string) => print!("{string}"),
        Err(error) => print!("Conversion Error: {error:?}")
    }
}
