use std::io;
use std::fmt::Write;

/* same as Q14003 */
fn binary_sreach(lis : &mut Vec<usize>, mut left : usize, mut right : usize, target : usize) -> usize {
    while left < right {
        let mid = (left + right) / 2;
        if target > lis[mid] {
            left = mid + 1;
        } else {
            right = mid;
        }
    } return right;
}

fn main() {
    let stdin = io::stdin();
    let mut out = String::new();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let size = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    stdin.read_line(&mut buf).unwrap();
    let mut nums = Vec::new(); nums.resize(size, 0);
    let mut nums_cnt = Vec::new(); nums_cnt.resize(size, 0);
    let mut iter = buf.split_whitespace();
    for i in 0..nums.len() {
        nums[i] = iter.next().unwrap().parse::<usize>().unwrap();
    } buf.clear();

    let mut lis = Vec::new();
    for i in 0..nums.len() {

        if lis.is_empty() || lis[lis.len()-1] < nums[i] {
            lis.push(nums[i]);
            nums_cnt[i] = lis.len();

        } else {
            let size = lis.len();
            let pos = binary_sreach(&mut lis , 0, size, nums[i]);
            lis[pos] = nums[i]; nums_cnt[i] = pos+1;
        }
    }

    let mut route = Vec::new();
    for i in (0..nums.len()).rev() {
        if nums_cnt[i] == lis.len() {
            route.push(nums[i]);
            lis.pop();
        }
    }

    write!(out, "{}\n", route.len()).unwrap();
    for i in route.into_iter().rev() {
        write!(out, "{} ", i).unwrap();
    } print!("{}", out);
}