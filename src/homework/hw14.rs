use std::collections::HashMap;
use std::io::{self, Write};

fn gray(n: u8) -> Vec<String> {
    fn generate(n: u8, memo: &mut HashMap<u8, Vec<String>>) -> Vec<String> {
        if let Some(cached) = memo.get(&n) {
            return cached.clone();
        }

        let result = match n {
            0 => vec![String::from("")],
            1 => vec![String::from("0"), String::from("1")],
            _ => {
                let prev = generate(n - 1, memo);
                let mut with_zero = prev.iter()
                    .map(|code| format!("0{}", code))
                    .collect::<Vec<_>>();
                let mut with_one = prev.iter()
                    .rev()
                    .map(|code| format!("1{}", code))
                    .collect::<Vec<_>>();
                with_zero.append(&mut with_one);
                with_zero
            }
        };

        memo.insert(n, result.clone());
        result
    }

    let mut memo = HashMap::new();
    generate(n, &mut memo)
}

fn main() {
    println!("Введіть кількість біт (0-8):");
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        if let Ok(bits) = input.trim().parse::<u8>() {
            if bits <= 8 {
                let codes = gray(bits);
                println!("Коди Ґрея довжини {}:", bits);
                for code in codes {
                    println!("{}", code);
                }
            } else {
                println!("Значення має бути в діапазоні 0–8.");
            }
        } else {
            println!("Невірний ввід. Очікується число.");
        }
    }
}
