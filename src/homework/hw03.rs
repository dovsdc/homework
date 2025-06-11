const HEIGHT: usize = 14;
const WIDTH: usize = 28;

fn main() {
    for row in 0..HEIGHT {
        for col in 0..=WIDTH {
            let ch = if row == 0 || row == HEIGHT - 1 {
                // Верхня та нижня рамка
                '*'
            } else if col == 0 || col == WIDTH {
                // Ліва та права рамка
                '*'
            } else if col == row * 2 || col == WIDTH - row * 2 {
                // Діагоналі, які формують конверт
                '*'
            } else {
                ' '
            };
            print!("{}", ch);
        }
        println!();
    }
}
