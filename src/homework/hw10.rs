use std::io;

/// Проверяет, является ли число палиндромом
fn is_palindrome(x: u32) -> bool {
    let original = x;
    let mut n = x;
    let mut rev: u32 = 0;

    while n > 0 {
        let digit = n % 10;
        // Безопасное умножение и сложение (без переполнения)
        rev = match rev.checked_mul(10).and_then(|v| v.checked_add(digit)) {
            Some(v) => v,
            None => return false, // Переполнение — не палиндром
        };
        n /= 10;
    }

    rev == original
}

fn main() {
    println!("Введите целое положительное число:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка чтения");

    let trimmed = input.trim();

    match trimmed.parse::<u32>() {
        Ok(number) => {
            if is_palindrome(number) {
                println!("{} — палиндром", number);
            } else {
                println!("{} — не палиндром", number);
            }
        }
        Err(_) => println!("Ошибка: введите корректное положительное целое число."),
    }
}
