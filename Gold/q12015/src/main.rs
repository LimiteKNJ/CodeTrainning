use std::io::{Write, BufWriter, self};


fn binary_sreach(lis : &mut Vec<usize>, mut left : usize, mut right : usize, target : usize) -> usize {

    while left < right {
        let mid = (left + right) / 2;

        if target > lis[mid] {
            left = mid + 1;

        } else {
            right = mid;
        }
    }

    return right;
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let size = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    stdin.read_line(&mut buf).unwrap();
    let mut nums = Vec::new(); nums.resize(size, 0);
    let mut iter = buf.split_whitespace();
    for i in 0..nums.len() {
        nums[i] = iter.next().unwrap().parse::<usize>().unwrap();
    } buf.clear();

    let mut lis = Vec::new();
    for i in 0..nums.len() {

        if lis.is_empty() || lis[lis.len()-1] < nums[i] {
            lis.push(nums[i]);

        } else {
            let size = lis.len();
            let pos = binary_sreach(&mut lis , 0, size, nums[i]);
            lis[pos] = nums[i];
        }
    } write!(out, "{}", lis.len()).unwrap();
}