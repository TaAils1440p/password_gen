use password_generator::generate_password;

#[test]
fn test_password_length() {
    let pw = generate_password(20, true, true);
    assert_eq!(pw.len(), 20);
}