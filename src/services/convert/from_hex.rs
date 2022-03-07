use std::string::String;

#[derive(Debug, PartialEq)]
pub enum ConvertError {
    HexStringToBytes,
    ToUtf8,
}

pub trait FromHex {
    fn hex_to_string(&self) -> Result<String, ConvertError>;
}

impl FromHex for str {
    fn hex_to_string(&self) -> Result<String, ConvertError> {
        let mut data = String::from(self);

        data = remove_delimiters(data);

        let decoded_bytes = match hex::decode(data) {
            Ok(decoded_bytes) => decoded_bytes,
            Err(_) => return Err(ConvertError::HexStringToBytes),
        };

        return match std::str::from_utf8(&decoded_bytes) {
            Ok(decoded_data) => Ok(decoded_data.to_string()),
            Err(_) => return Err(ConvertError::ToUtf8),
        }
    }
}

fn remove_delimiters(data: String) -> String {
    data.replace(" ", "")
}

#[cfg(test)]
mod test {
    use super::{FromHex, ConvertError};
    use std::string::String;

    /// Common way to test positive test cases
    macro_rules! positive_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    // Aggregate
                    let (input_data, expected) = $value;

                    // Act
                    let actual = input_data.hex_to_string().unwrap();

                    // Assert
                    assert_eq!(actual, expected)
                }
            )*
        }
    }

    /// Negative tests
    macro_rules! negative_tests {
        ($($name:ident: $values:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    // Aggregate
                    let (input_data, expected) = $values;

                    // Act
                    let actual = input_data.hex_to_string().unwrap_err();

                    // Assert
                    assert_eq!(actual, expected)
                }
            )*
        }
    }

    positive_tests! {
        empty_string: (String::new(), ""),
        digits: ("30 31 32 33 34 35 36 37 38 39", "0123456789"),
        lowercase_ascii: (
            "61 62 63 64 65 66 67 68 69 6a 6b 6c 6d 6e 6f 70 71 72 73 74 75 76 77 78 79 7a",
            "abcdefghijklmnopqrstuvwxyz"),
        uppercase_ascii: (
            "41 42 43 44 45 46 47 48 49 4a 4b 4c 4d 4e 4f 50 51 52 53 54 55 56 57 58 59 5a",
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
        utf8_1byte_first_code_point: ("00", "\u{0000}"),
        utf8_1byte_last_code_point: ("7f", "\u{007F}"),
        utf8_2bytes_first_code_point: ("c2 80", "\u{0080}"),
        utf8_2bytes_last_code_point: ("df bf", "\u{07FF}"),
        utf8_3bytes_first_code_point: ("e0 a0 80", "\u{0800}"),
        utf8_3bytes_last_code_point: ("EF BF BF", "\u{FFFF}"),
        utf8_4bytes_first_code_point: ("F0 90 80 80", "\u{10000}"),
        utf8_4bytes_last_code_point: ("F4 8F BF BF", "\u{10FFFF}"),
    }

    negative_tests!{
        utf8_neg_1byte_out_of_range: ("80", ConvertError::ToUtf8),
        utf8_neg_2bytes_out_of_range_to_low: ("c2 7F", ConvertError::ToUtf8),
        utf8_neg_2bytes_out_of_range_to_high: ("df c0", ConvertError::ToUtf8),
        utf8_neg_3bytes_out_of_range_to_low: ("e0 a0 7F", ConvertError::ToUtf8),
        utf8_neg_3bytes_out_of_range_to_high: ("EF BF C0", ConvertError::ToUtf8),
        utf8_neg_4bytes_out_of_range_to_low: ("F0 90 80 7F", ConvertError::ToUtf8),
        utf8_neg_4bytes_out_of_range_to_high: ("F4 8F BF C0", ConvertError::ToUtf8),

        hex_neg_odd_char: ("8", ConvertError::HexStringToBytes),
        hex_neg_invalid_char: ("FG", ConvertError::HexStringToBytes),
    }
}
