fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let num = buf.trim().parse::<usize>().unwrap();
    let mut result = String::new();

    for i in 0..num*2 {
        if i < num {
            for _ in 1..num-i {
                result.push(' ');

            } for _ in 0..2*i+1 {
                result.push('*');
            }

        } else {
            for _ in 0..(i-num)+1 {
                result.push(' ');

            } for _ in ((2*(i-num)+3)..(num*2)).rev() {
                result.push('*');
            }
        } result.push('\n');
    }

    print!("{}", result);
}