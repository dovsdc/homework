use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let letters = ['m', 'u', 'x', 'a', 's', 'l', 'o', 'n'];
    let digits = (1..=8).collect::<Vec<_>>();

    let mut count = 0;

    for perm in digits.iter().permutations(8).unique() {
        let mut map = HashMap::new();
        for (ch, &digit) in letters.iter().zip(perm.iter()) {
            map.insert(ch, digit);
        }

        let m = map[&'m'];
        let u = map[&'u'];
        let x = map[&'x'];
        let a = map[&'a'];
        let s = map[&'s'];
        let l = map[&'l'];
        let o = map[&'o'];
        let n = map[&'n'];

        let muxa = m * 1000 + u * 100 + x * 10 + a;
        let slon = s * 1000 + l * 100 + o * 10 + n;

        if muxa * a == slon {
            count += 1;
            println!(
                "  {}{}{}{}\n×     {}\n-------\n  {}{}{}{}",
                m, u, x, a, a, s, l, o, n
            );
            println!();
        }
    }

    println!("Total solutions: {}", count);
}
