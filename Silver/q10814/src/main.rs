use std::io;
use io::Write;

#[derive(Debug)]
struct people {
    age : usize,
    name : String
}

fn create_user (age : usize, name : String) -> people {
    people { age: age, name: name}
}

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut people_vec : Vec<people> = Vec::new();
    let mut buf = String::new();
    
    stdin.read_line(&mut buf).unwrap();
    let num = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    for _ in 0..num {
        stdin.read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let user = create_user(
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().to_string()
        ); buf.clear();
        people_vec.push(user);
    } 

    people_vec.sort_by(|a,b| a.age.cmp(&b.age));
    for i in people_vec {
        writeln!{out, "{} {}", i.age, i.name}.unwrap();
    }
}