#[test]
fn test_ceasar_cipher() {
    use ceasar_macro::caesar_cipher;

    let result = caesar_cipher!("Hello, World!", 3);
    let expected = "Khoor, Zruog!";
    assert_eq!(result, expected);
    assert_eq!("", caesar_cipher!("", 3));
    assert_ne!("a", caesar_cipher!("a", 3));
    assert_eq!("a", caesar_cipher!("a", 0));
}
