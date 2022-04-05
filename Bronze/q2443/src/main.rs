fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let num = buf.trim().parse::<usize>().unwrap();
    let mut result = String::new();

    for i in (0..num).rev() {
        for _ in 1..num-i {
            result.push(' ');

        } for _ in 0..2*i+1 {
            result.push('*');
 
        } result.push('\n');
    }

    print!("{}", result);
}