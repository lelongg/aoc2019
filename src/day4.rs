use itertools::Itertools;

fn main() {
    let result = (156_218..=652_527)
        .filter(|&password| is_valid_password(password))
        .count();
    println!("{}", result)
}

fn is_valid_password(password: u32) -> bool {
    let mut last_sequence_length = None;
    let mut has_pair = false;
    for (digit, next_digit) in password.to_string().chars().tuple_windows() {
        if digit > next_digit {
            return false;
        }
        if digit == next_digit {
            last_sequence_length = last_sequence_length.map(|len| len + 1).or(Some(2));
        } else {
            if let Some(2) = last_sequence_length {
                has_pair = true;
            }
            last_sequence_length = None;
        }
    }
    has_pair || last_sequence_length == Some(2)
}

#[test]
fn test() {
    assert!(is_valid_password(112_233));
    assert!(!is_valid_password(123_444));
    assert!(is_valid_password(111_122));
}
