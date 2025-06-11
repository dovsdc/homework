use std::io;

/// Сдвигает строку циклически на `n` позиций вправо.
/// Отрицательное `n` означает сдвиг влево.
fn rotate2(s: String, n: isize) -> String {
    let len = s.chars().count();
    if len == 0 {
        return s;
    }

    let shift = (n.rem_euclid(len as isize)) as usize;
    if shift == 0 {
        return s;
    }

    let chars: Vec<char> = s.chars().collect();
    let split_at = len - shift;

    let mut rotated = String::with_capacity(s.len());
    rotated.extend(chars[split_at..].iter());
    rotated.extend(chars[..split_at].iter());
    rotated
}

fn main() {
    let mut input = String::new();
    println!("Введите строку:");
    io::stdin().read_line(&mut input).expect("Ошибка чтения строки");
    let s = input.trim().to_string();

    let mut shift_input = String::new();
    println!("Введите сдвиг (может быть отрицательным):");
    io::stdin().read_line(&mut shift_input).expect("Ошибка чтения числа");

    let shift: isize = match shift_input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Ошибка: введите корректное число.");
            return;
        }
    };

    let rotated = rotate2(s, shift);
    println!("Результат сдвига: {}", rotated);
}
