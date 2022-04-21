/* code 32128449 // 알고리즘 자체는 전체적으로 비슷한 듯 */
use std::io::{Write, BufWriter, self};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut html_src : &str = &buf.trim();

    let mut p_text = None;
    while !html_src.is_empty() {

        // read Token until ">" and delete String
        let close = html_src.find(">").unwrap();
        let name_tag = &html_src[1..close];
        html_src = &html_src[close + 1..];

        // read Token until " to Write Title
        if let Some(quote) = name_tag.find("\"") {
            let mut title = &name_tag[quote + 1..];
            title = &title[..title.len() - 1];
            writeln!(out, "title : {}", title).unwrap();

        } else if name_tag == "p" {
            p_text = Some(String::new());

        // read contexts in p tags
        } else if name_tag == "/p" {
            let text = p_text.take().unwrap();
            let mut text = text.trim();
            loop { // Delete WhiteSpace
            if let Some(white_begin) = text.find(' ') {
                out.write(text[..white_begin].as_bytes()).unwrap();
                out.write(b" ").unwrap();
                text = &text[white_begin + 1..];
                    if let Some(white_end) = text.find(|c| c != ' ') {
                        text = &text[white_end..];
                    } else { break; }
                } else { break; }
            } writeln!(out, "{}", text).unwrap(); 
        }
    
        // read Token "<" to check Opened p tags
        if let Some(buf) = &mut p_text {
            let next_open = html_src.find("<").unwrap();
            let next = &html_src[..next_open];
            html_src = &html_src[next_open..];
            buf.push_str(next);
        }
    }
}