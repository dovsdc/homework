pub fn draw_tree(levels: usize) {
    let max_stars = 1 + 2 * levels; // кількість зірок у найширшому рядку (останній трикутник)
    
    for level in 1..=levels {
        for row in 0..=level {
            let stars = 1 + 2 * row;
            let padding = (max_stars - stars) / 2;
            let line = " ".repeat(padding) + &"*".repeat(stars);
            println!("{}", line);
        }
    }

    // Додатковий рядок основи
    let trunk = "*".repeat(max_stars);
    println!("{}", trunk);
}

fn main() {
    let levels = 5; // змінюй тут кількість трикутників
    draw_tree(levels);
}
