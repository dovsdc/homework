use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

fn simple_random(seed: &mut u64) -> u32 {
    // Простейший линейный конгруэнтный генератор
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
    ((*seed >> 32) as u32) % 91 + 10  // в диапазоне [10, 100]
}

fn main() {
    let mut seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    let data_len = 20;
    let mut data = Vec::new();

    for _ in 0..data_len {
        data.push(simple_random(&mut seed) as i32);
    }

    println!(
        "indexes: {}",
        (0..data_len)
            .map(|i| format!("{:>2}.", i))
            .collect::<Vec<_>>()
            .join("  ")
    );

    println!("data:   {:?}", data);

    if data.len() < 2 {
        return;
    }

    let (mut min_sum, mut min_index) = (i32::MAX, 0);
    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    println!(
        "indexes: {}\\__ __/{}",
        " ".repeat(min_index * 4),
        " ".repeat((data.len() - min_index - 2) * 4)
    );

    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[min_index],
        data[min_index + 1],
        min_sum,
        min_index,
        min_index + 1
    );
}
