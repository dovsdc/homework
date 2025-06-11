const HEIGHT: usize = 6; // Висота від центру до верхівки (включно)
const WIDTH: usize = 2 * HEIGHT - 1;

fn main() {
    for row in 0..(2 * HEIGHT - 1) {
        for col in 0..WIDTH {
            let midpoint = HEIGHT - 1;
            let dist = if row < HEIGHT {
                row
            } else {
                2 * HEIGHT - 2 - row
            };
            let start = midpoint - dist;
            let end = midpoint + dist;

            let ch = if col >= start && col <= end {
                '*'
            } else {
                ' '
            };
            print!("{}", ch);
        }
        println!();
    }
}
