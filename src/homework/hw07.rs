use std::io::{self, Write};

fn invert_the_case(s: String) -> String {
    let mut result = String::with_capacity(s.len());

    for c in s.chars() {
        if c.is_lowercase() {
            for up in c.to_uppercase() {
                result.push(up);
            }
        } else if c.is_uppercase() {
            for low in c.to_lowercase() {
                result.push(low);
            }
        } else {
            result.push(c);
        }
    }

    result
}

fn main() {
    print!("Введите строку: ");
    io::stdout().flush().unwrap(); // выводим приглашение сразу

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода");

    // Удаляем возможный перевод строки
    let trimmed = input.trim_end();

    let result = invert_the_case(trimmed.to_string());
    println!("Инвертированный регистр: {}", result);
}
