fn main() {
    const N: usize = 6; // задаёт количество строк в верхней части ромба
    let mut diamond = String::new();

    // Формируем верхнюю часть ромба (включая центральную строку)
    for i in 1..=N {
        let spaces = " ".repeat(N - i);
        let stars = "*".repeat(2 * i - 1);
        diamond.push_str(&format!("{}{}\n", spaces, stars));
    }
    // Формируем нижнюю часть ромба
    for i in (1..N).rev() {
        let spaces = " ".repeat(N - i);
        let stars = "*".repeat(2 * i - 1);
        diamond.push_str(&format!("{}{}\n", spaces, stars));
    }

    // Выводим результат одним вызовом print!
    print!("{}", diamond);
}
