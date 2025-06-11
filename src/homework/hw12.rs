use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{self, Write};

fn calculate_balance_steps(variable_list: &Vec<u32>) -> isize {
    let total_value: u32 = variable_list.iter().sum();
    let count = variable_list.len() as u32;

    if total_value % count != 0 {
        return -1;
    }

    let average = total_value / count;
    let mut step_counter = 0;
    let mut imbalance = 0;

    for &value in variable_list {
        imbalance += value as i32 - average as i32;
        step_counter += imbalance.abs() as usize;
    }

    step_counter as isize
}

// Простая реализация генератора случайных чисел
fn simple_random(seed: &mut u64) -> u32 {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
    ((*seed >> 32) as u32) % 100 + 1
}

fn generate_balanced_vector(size: usize, seed: &mut u64) -> Vec<u32> {
    let average_value = simple_random(seed);
    let mut generated_vector = vec![average_value; size];

    for i in 0..size / 2 {
        let variation = simple_random(seed) % (average_value.min(10) + 1);
        generated_vector[i] += variation;
        generated_vector[size - 1 - i] = generated_vector[size - 1 - i].saturating_sub(variation);
    }

    generated_vector
}

fn main() {
    let example_one = vec![8, 2, 2, 4, 4];
    let result_one = calculate_balance_steps(&example_one);
    println!("Example 1: {:?} => Moves: {}", example_one, result_one);

    let example_two = vec![9, 3, 7, 2, 9];
    let result_two = calculate_balance_steps(&example_two);
    println!("Example 2: {:?} => Moves: {}", example_two, result_two);

    let example_three = vec![1, 1, 1, 1, 6];
    let result_three = calculate_balance_steps(&example_three);
    println!("Example 3: {:?} => Moves: {}", example_three, result_three);

    // Получение seed от системного времени
    let mut seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    let generated = generate_balanced_vector(10, &mut seed);
    println!("Generated vector: {:?}", generated);
    let generated_result = calculate_balance_steps(&generated);
    println!("Moves required to balance generated vector: {}", generated_result);

    // Ввод пользовательских данных
    println!("\nВведите числа через пробел для расчета:");
    print!("> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        let input_vec: Vec<u32> = input
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();
        if input_vec.is_empty() {
            println!("Неверный ввод.");
        } else {
            let res = calculate_balance_steps(&input_vec);
            println!("Moves required: {}", res);
        }
    }
}
