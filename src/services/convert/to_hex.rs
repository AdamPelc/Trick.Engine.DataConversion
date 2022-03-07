// Might be useful later: https://stackoverflow.com/a/25265605

use std::ops::Not;

/// Converts the input string to hexadecimal bytes representation in string
/// separated by the specified delimiter.
pub fn convert_to_hex(input_data: &str) -> String {
    if is_input_data_valid(input_data).not() {
        return String::new();
    }
    let mut result = allocate_string(input_data.bytes().len());
    result.fill_string_from(&mut input_data.bytes());

    result
}

use std::str::Bytes;

fn is_input_data_valid(input_data: &str) -> bool {
    if 0 == input_data.bytes().len() {
        return false;
    }
    true
}

fn allocate_string(input_data_length: usize) -> String {
    let mut result = String::new();

    // Each hexadecimal value takes two bytes.
    let hexes_size = input_data_length * 2;
    // Last space from string is trimmed.
    let spaces_size = input_data_length - 1;

    result.reserve(hexes_size + spaces_size);

    result
}

trait Fill {
    fn fill_string_from(&mut self, byte_iter: &mut Bytes<'_>);
}

impl Fill for String {
    fn fill_string_from(&mut self, byte_iter: &mut Bytes<'_>) {
        for _ in 0..(byte_iter.len() - 1) {
            self.push_str(&format!("{:0>2x} ", byte_iter.next().unwrap()));
        }
        self.push_str(&format!("{:0>2x}", byte_iter.next().unwrap()));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Common way to test positive test cases
    macro_rules! positive_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    // Aggregate
                    let (input_data, expected) = $value;

                    // Act
                    let actual = convert_to_hex(input_data);

                    // Assert
                    assert_eq!(actual, expected)
                }
            )*
        }
    }

    positive_tests! {
        empty_string: (&String::new(), ""),
        digits: ("0123456789", "30 31 32 33 34 35 36 37 38 39"),
        lowercase_ascii: (
            "abcdefghijklmnopqrstuvwxyz",
            "61 62 63 64 65 66 67 68 69 6a 6b 6c 6d 6e 6f 70 71 72 73 74 75 76 77 78 79 7a"),
        uppercase_ascii: (
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            "41 42 43 44 45 46 47 48 49 4a 4b 4c 4d 4e 4f 50 51 52 53 54 55 56 57 58 59 5a"),
        utf8_1byte_first_code_point: ("\u{0000}", "00"),
        utf8_1byte_last_code_point: ("\u{007F}", "7f"),
        utf8_2bytes_first_code_point: ("\u{0080}", "c2 80"),
        utf8_2bytes_last_code_point: ("\u{07FF}", "df bf"),
        utf8_3bytes_first_code_point: ("\u{0800}", "e0 a0 80"),
        utf8_3bytes_last_code_point: ("\u{FFFF}", "ef bf bf"),
        utf8_4bytes_first_code_point: ("\u{10000}", "f0 90 80 80"),
        utf8_4bytes_last_code_point: ("\u{10FFFF}", "f4 8f bf bf"),
    }

    #[test]
    fn returned_string_size() {
        // Act
        let response = convert_to_hex("a");
        // Assert
        assert!(response.len() <= response.capacity());
    }
}
