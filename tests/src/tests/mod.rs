#[test]
fn test_ceasar_cipher() {
    use ceasar_macro::caesar_cipher;

    let result = caesar_cipher!("Hello, World!", 3);
    let expected = "Khoor, Zruog!";
    assert_eq!(result, expected);
}
