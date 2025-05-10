fn charpix(x: usize, y: usize, width: usize, height: usize) -> char {
    let border = x == 0 || x == width - 1 || y == 0 || y == height - 1;
  
    let lposition = (y * (width - 1)) / (height - 1);
    let rposition = ((height - 1 - y) * (width - 1)) / (height - 1);

    let diagonal = x == lposition || x == rposition;
    
    if border || diagonal {
        '*'
    } else {
        ' '
    }
}

fn main() {
    const WIDTH: usize = 28;
    const HEIGHT: usize = 10;
    
    let mut output = String::new();
    
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            output.push(charpix(x, y, WIDTH, HEIGHT));
        }
        output.push('\n');
    }
    print!("{}", output);
}
