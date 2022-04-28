use std::io;
use std::fmt::Write;

fn main() {
    let stdin = io::stdin();
    let mut out= String::new();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let case = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    for i in 1..=case {
        stdin.read_line(&mut buf).unwrap();
        let mut url_src = buf.trim();

        write!(out, "URL #{}\n", i).unwrap();
        let protocol_signal = url_src.find("://").unwrap();
        let protocol = &url_src[0..protocol_signal];
        write!(out, "Protocol = {}\n", protocol).unwrap();
        url_src = &url_src[protocol_signal+3..];

        if let Some(slash_signal) = url_src.find("/"){
            let host_name = &url_src[0..slash_signal];

            let mut iter = host_name.split(":");
            let host = iter.next().unwrap();
            write!(out, "Host     = {}\n", host).unwrap();
            let port = match iter.next(){
                Some(n) => n,
                None => "<default>",
            }; write!(out, "Port     = {}\n", port).unwrap();

            if !url_src.trim().is_empty() {
                let path = &url_src[slash_signal+1..];
                write!(out, "Path     = {}\n", path).unwrap();
            } else {
                write!(out, "Path     = <default>").unwrap();
            }

        } else {
            let host_name = &url_src;

            let mut iter = host_name.split(":");
            let host = iter.next().unwrap();
            write!(out, "Host     = {}\n", host).unwrap();
            let port = match iter.next(){
                Some(n) => n,
                None => "<default>",
            }; write!(out, "Port     = {}\n", port).unwrap();
            write!(out, "Path     = <default>\n").unwrap();
        } buf.clear(); write!(out,"\n").unwrap();

    } print!("{}",out);
}