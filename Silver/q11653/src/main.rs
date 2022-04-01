use std::io;
use io::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut num = buf.trim().parse::<usize>().unwrap();
    let stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let mut k = 2;

    let mut result = String::new();
    while num != 1 {
		if num % k == 0 {
			result.push_str(&k.to_string());
            result.push_str("\n");
			num /= k;
		}
		else {
			k += 1;
		}
	}

    write!(out, "{}", result).unwrap();
}