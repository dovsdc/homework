use std::collections::HashSet;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    a: Point,
    b: Point,
}

fn area_occupied(rects: &Vec<Rectangle>) -> i32 {
    let mut occupied_points: HashSet<Point> = HashSet::new();

    for rect in rects {
        let x_min = rect.a.x.min(rect.b.x);
        let x_max = rect.a.x.max(rect.b.x);
        let y_min = rect.a.y.min(rect.b.y);
        let y_max = rect.a.y.max(rect.b.y);

        for x in x_min..x_max {
            for y in y_min..y_max {
                occupied_points.insert(Point { x, y });
            }
        }
    }

    occupied_points.len() as i32
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

fn run_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    println!("Окупована площа (тестові дані): {}", occupied);
    assert_eq!(occupied, 60);
    println!("Тест пройдено успішно!");
}

fn read_user_input() -> Vec<Rectangle> {
    let mut rectangles = Vec::new();
    println!("Введіть кількість прямокутників:");
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let num: usize = match line.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Некоректне число.");
            return rectangles;
        }
    };

    for i in 0..num {
        println!("Прямокутник {}: Введіть x1 y1 x2 y2 через пробіл:", i + 1);
        line.clear();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line).unwrap();
        let parts: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        if parts.len() != 4 {
            println!("Пропущено або занадто багато чисел, пропускаю прямокутник.");
            continue;
        }

        let rect = Rectangle {
            a: Point { x: parts[0], y: parts[1] },
            b: Point { x: parts[2], y: parts[3] },
        };
        rectangles.push(rect);
    }

    rectangles
}

fn main() {
    run_test();

    println!("\n--- Власний приклад користувача ---");
    let user_rects = read_user_input();
    if user_rects.is_empty() {
        println!("Немає даних для обчислення.");
        return;
    }

    let area = area_occupied(&user_rects);
    println!("Окупована площа: {}", area);
}
