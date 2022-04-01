fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let cases = buf.trim().parse::<usize>().unwrap();

    for _ in 0..cases {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let nums = buf.split_whitespace()
                    .map(|s|s.trim().parse().unwrap())
                    .collect::<Vec<usize>>();

        let a = nums[0];
        let b = nums[1];
        let mut result = 1;

        for _ in 0..b {
            result = (result * a) % 10;
        }

        if result == 0 {
            print!("10\n");
        } else {
            print!("{}\n", result);
        }
    }
}