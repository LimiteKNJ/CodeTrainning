use std::io;

fn main() {
    let mut count = 10;
    let mut res : Vec<usize> = vec![];

    for _i in 0..10{
        let mut text = String::new();
        io::stdin().read_line(&mut text).expect("no input");
        let num : usize = text.trim().parse().expect("Error");

        res.push(num % 42);
    }

    for i in 0..10 {
        for j in i+1..10 {
            if cmp(res[i], res[j]) {
                count -= 1;
                break;
            }
        }
    }

    println!("{}", count);
}

fn cmp(x : usize, y : usize) -> bool{
    
    let b_res : bool;
    if x == y {
        b_res = true;
    } else {
        b_res = false;
    }
    return b_res;
}
