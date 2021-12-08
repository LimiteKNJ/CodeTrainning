use std::io;
fn main(){
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("no Input");
    let num : usize = s.trim().parse().expect("Error.");
    let mut out = String::new();

    for i in 1..num+1{ 
        let t = i.to_string();
        out.push_str(&t);
        out.push_str("\n");
    }

    println!("{}",out);
}