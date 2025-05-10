use rand::Rng;

pub fn run() {
    let mut rng = rand::thread_rng();
    let data: Vec<i32> = (0..20).map(|_| rng.gen_range(10..=100)).collect();

    println!("indexes: {}", (0..20).map(|i| format!("{:>2}.", i)).collect::<Vec<_>>().join("  "));
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
