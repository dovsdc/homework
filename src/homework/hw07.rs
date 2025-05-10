fn invert_the_case(s: String) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        if c.is_lowercase() {
            for up in c.to_uppercase() {
                result.push(up);
            }
        }
        else if c.is_uppercase() {
            for low in c.to_lowercase() {
                result.push(low);
            }
        }
        else {
            result.push(c);
        }
    }
    result
}
#[test]
fn test_invert_the_case() {
    let data = [
        ("Hello",  "hELLO"),
        ("Привет","пРИВЕТ"),
    ];

    for &(input, expected) in &data {
        assert_eq!(invert_the_case(input.to_string()), expected.to_string());
        assert_eq!(invert_the_case(expected.to_string()), input.to_string());
    }
}
