use std::io;
use io::Write;

fn main() {

    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let num_cnt = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    let mut nums = Vec::new();
    for _ in 0..num_cnt {
        stdin.read_line(&mut buf).unwrap();
        nums.push(buf.trim().parse::<isize>().unwrap());
        buf.clear();
    
    } nums.sort();

    let mut sum = 0;
    for n in &nums {
        sum += n;
    } let avg = (sum as f64 / num_cnt as f64).round() as isize;
      
    let mid = nums.get(((num_cnt / 2) as f64).ceil() as usize).unwrap();

    let mut mode = -4001;
    let mut max_count = 0;
    let mut count_num = vec![0 as usize; 8002];
    for n in &nums {
        count_num[(n + 4000) as usize] += 1;
    }
    
    let mut mode_2nd = true;
    for i in 0..count_num.len() {
        if count_num[i] as isize > max_count {
            max_count = count_num[i] as isize;
            mode = i as isize - 4000 as isize;
            mode_2nd = false;
        } else if count_num[i] as isize == max_count && !mode_2nd {
            mode_2nd = true;
            mode = i as isize - 4000 as isize;
        }
    }
    
    let range = nums.get(num_cnt - 1).unwrap() - nums.get(0).unwrap();

    writeln!(out, "{} {} {} {}", avg, mid, mode, range).unwrap();
}