use std::io::{Write, BufWriter, self};

fn main() {
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    write!(out, "강한친구 대한육군\n강한친구 대한육군").unwrap();
}