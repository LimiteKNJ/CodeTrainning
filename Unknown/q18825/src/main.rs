fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut num = buf.trim().parse::<usize>().unwrap();
    let mut k = 2;

    while num != 1 {
		if num % k == 0 {
			print!("{} ", k);
			num /= k;
		}
		else {
			k += 1;
		}
	}
}