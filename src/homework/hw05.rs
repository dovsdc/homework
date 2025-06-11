use std::io::{self, Write};

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    println!("Обчислення найбільшого спільного дільника (НСД / GCD)");
    println!("Введіть два числа через пробіл:");

    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        let parts: Vec<u32> = input
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if parts.len() == 2 {
            let result = gcd(parts[0], parts[1]);
            println!("НСД({}, {}) = {}", parts[0], parts[1], result);
        } else {
            println!("Помилка: потрібно ввести рівно два цілих числа.");
        }
    } else {
        println!("Не вдалося прочитати введення.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_cases() {
        let data = [
            ((24, 60), 12),
            ((15, 9), 3),
            ((15, 6), 3),
            ((140, 40), 20),
            ((24, 16), 8),
            ((100, 10), 10),
            ((120, 80), 40),
            ((80, 120), 40),
            ((100, 20), 20),
            ((37, 11), 1),
            ((120, 90), 30),
        ];
        for &((a, b), expected) in data.iter() {
            assert_eq!(gcd(a, b), expected, "gcd({}, {}) failed", a, b);
        }
    }
}
