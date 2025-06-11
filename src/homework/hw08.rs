use std::io;

/// Проверяет, является ли число простым
fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as u32;
    for i in 2..=sqrt_n {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    println!("Введите целое число:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода");

    let trimmed = input.trim();

    match trimmed.parse::<u32>() {
        Ok(number) => {
            if is_prime(number) {
                println!("{} — простое число", number);
            } else {
                println!("{} — не простое число", number);
            }
        }
        Err(_) => println!("Ошибка: введите корректное положительное число."),
    }
}
