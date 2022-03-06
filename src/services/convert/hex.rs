// Might be useful later: https://stackoverflow.com/a/25265605

/// Converts the input string to hexadecimal bytes separated by the specified delimiter.
pub fn convert_to_hex(input_data: &str) -> String {
    let mut result = String::new();
    let size = input_data.len() * 3; // Each char takes 2 spots + space between each char.
    result.reserve(size);

    for char in input_data {
        println!("Char: {}", char);
    }

    result
}

#[test]
fn convert_to_hex_test() {
    // Assert
    assert_eq!(convert_to_hex("a9z"), "61 39 7a");
}
