use rustere;

#[test]
fn test_odd() {
    assert!(rustere::is_odd(654541));
}

#[test]
fn test_even() {
    assert!(!rustere::is_odd(98768));
}

#[test]
fn test_zero() {
    assert!(!rustere::is_odd(0));
}
