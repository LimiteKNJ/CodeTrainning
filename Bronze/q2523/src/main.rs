fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let num = buf.trim().parse::<usize>().unwrap();
    let mut result = String::new();

    for i in 0..num*2 {
        if i < num {
            for _ in 0..i+1 {
                result.push('*');
            }

        } else {
            for _ in (i-num+1..num).rev() {
                result.push('*');
            }
        }
        result.push('\n');
    }

    print!("{}", result);
}
