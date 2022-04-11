/* Rust Answer 

use std::{io::*, fs::*, os::unix::io::*};

fn solve(mut doc: &str, out: &mut impl Write) {
    let mut p_buf = None;
    while !doc.is_empty() {
        let close = doc.find('>').unwrap();
        let name = &doc[1..close];
        doc = &doc[close + 1..];
        if let Some(quote) = name.find("\"") {
            let mut title = &name[quote + 1..];
            title = &title[..title.len() - 1];
            writeln!(out, "title : {}", title).unwrap();
        } else if name == "p" {
            p_buf = Some(String::new());
        } else if name == "/p" {
            let text = p_buf.take().unwrap();
            let mut text = text.trim();
            loop {
                if let Some(white_begin) = text.find(' ') {
                    out.write(text[..white_begin].as_bytes()).unwrap();
                    out.write(b" ").unwrap();
                    text = &text[white_begin + 1..];
                    if let Some(white_end) = text.find(|c| c != ' ') {
                        text = &text[white_end..];
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            writeln!(out, "{}", text).unwrap();
        }
        if let Some(buf) = &mut p_buf {
            let next_open = doc.find("<").unwrap();
            let text = &doc[..next_open];
            doc = &doc[next_open..];
            buf.push_str(text);
        }
    }
}

fn main() {
    let mut stdin = stdin();
    let mut stdout = BufWriter::new(unsafe { File::from_raw_fd(1) });
    let mut document = String::new();
    stdin.read_to_string(&mut document).unwrap();
    solve(document.trim(), &mut stdout);
}

*/

/*  Solving read Char to Run */ // Time OUT!
/* 
use std::io;
use io::Write;

fn main() {
    let mut bf = String::new();
    std::io::stdin().read_line(&mut bf).expect("Err");
    let html = bf.trim().chars().collect::<Vec<_>>();
    bf.clear();

    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let tags = find_start_end_tag(html.clone());
    let starts = tags.0;
    let ends = tags.1;
    let mut iter = 0;

    let mut result = String::new();
    let mut p_start = 0;
    let mut p_end = 0;

    for i in starts {
        let start = i;
        let end = ends[iter];
        let mut str = String::new();

        for j in start..end+1 {
            str.push(html[j]);
        }

        if str.contains("<div title=") {
            result.push_str(&div(str.clone()));
        }

        else if str.contains("</p>") {
            p_end = end;

            for k in p_start..p_end+1 {
                str.push(html[k]);
            } result.push_str(&p(str.clone()));
        
        } else if str.contains("<p>") {
            p_start = start;
        }

        iter += 1;
    }

    writeln!(out, "{}", result).expect("Err");
}

fn find_start_end_tag (html : Vec<char>) -> (Vec<usize>, Vec<usize>) {
    
    let mut start_list : Vec<usize> = Vec::new();
    let mut end_list : Vec<usize> = Vec::new();
    
    let mut iter = 0;

    for i in html {
        if i == '<' {
            start_list.push(iter);
        } else if i == '>' {
            end_list.push(iter);
        } iter += 1;
    }

    return (start_list, end_list);
}

fn div(str : String) -> String {
    
    if str != "</div>" {
        let temp = str.chars().collect::<Vec<char>>();
        let mut result : String = "title : ".to_string();
        let mut c_flag = false;
        let mut c_start = 0;
        let mut c_end = 0;
        
        for i in 0..temp.len() {
            if temp[i] == '"' && !c_flag {
                c_start = i;
                c_flag = true;
            } else if temp[i] == '"' && c_flag {
                c_end = i;
                c_flag = false;
            }
        }
        
        for i in c_start+1..c_end{
            result.push(temp[i]);
            if i == c_end-1 {
                result.push('\n');
            }
        }

        return result;

    } else {
        return "".to_string();
    }
}

fn p(str : String) -> String {

    let tags = find_start_end_tag(str.chars().collect());
    let mut result = String::new();
    let mut texts = String::new();
    let starts = tags.0;
    let ends = tags.1;
    let mut iter = 0;

    for i in starts {        
        if iter == 0 {
            iter += 1;
            continue;

        } else {
            let temp = str.chars().collect::<Vec<char>>();
            let start = i;
            let end = ends[iter-1];

            for j in end+1..start{
                texts.push(temp[j]);
            }
            result = texts.split_whitespace().
                                map(|s|s.trim().parse().expect("Err"))
                                .collect::<Vec<String>>().join(" ");
            iter += 1;
        }
    }

    result.push('\n');
    return result;
}

*/


// Previous Codes :: My Code

const NULL : usize = 0;
const OPEN_HTML : usize = 101;
const CLOSE_HTML : usize = 102;
const DIV_NOT_OPEN : usize = 111;
const DIV_OPEN : usize = 112;
const DIV_CLOSE : usize = 113;
const P_OPEN : usize = 121;
const P_CLOSE : usize = 122;
const B_OPEN : usize = 131;
const B_CLOSE : usize = 132;
const ITALIC_OPEN : usize = 141;
const ITALIC_CLOSE : usize = 142;
const BR_FLAGS : usize = 150;
const OTHER_FLAGS : usize = 155;
const OTHER_FLAGS_CLOSE : usize = 156;

fn html_tag_check (tags : String) -> usize {

    let mut flags = NULL;
    if tags == "main" { flags = OPEN_HTML; }
    if tags == "/main" { flags = CLOSE_HTML; }
    if tags == "div" { flags = DIV_NOT_OPEN; }
    if tags == "div title" { flags = DIV_OPEN; }
    if tags == "/div"{ flags = DIV_CLOSE; }
    if tags == "p" { flags = P_OPEN; }
    if tags == "/p" { flags = P_CLOSE; }
    if tags == "b" { flags = B_OPEN; }
    if tags == "/b"{ flags = B_CLOSE; }
    if tags == "i" { flags = ITALIC_OPEN; }
    if tags == "/i"{ flags = ITALIC_CLOSE; }
    if tags == "br "{ flags = BR_FLAGS; }

    return flags;
}



fn main() {
    let mut bf = String::new();
    std::io::stdin().read_line(&mut bf).expect("Err");
    let mut str = bf.trim().chars().collect::<Vec<char>>();
    str.push('\n');

    let mut temp = String::new();
    let mut iter = 0;

    let mut p_open = false;
    let mut c_open = false;
    let mut err = false;

    let mut html = false;
    let mut div = false;
    let mut p = false;
    let mut bold = false;
    let mut italic = false;
    let mut tag_open = false;
    let mut trg = false;

    let mut start_p = false;
    let mut end_p = false;
    let mut is_more_whitespace = false;

    let mut stack : Vec<usize> = Vec::new();
    let mut result = String::new();

    while str[iter] != '\n' {

        if !p_open && str[iter] == '<' {
            p_open = true;
        
        } else if p_open && str[iter] == '>' {

            let tags = temp.clone();
            let flags = html_tag_check(tags);
            if flags != NULL {
                stack.push(flags);
            } temp.clear();
            trg = true;
            p_open = false;
/*
            let tags = temp.clone();
            if !p {
                let flags = html_tag_check(tags);
                if flags != NULL {
                    stack.push(flags);
                } temp.clear();
                trg = true;
                p_open = false;

            } else {
                if tags.starts_with("/") {
                    stack.push(OTHER_FLAGS_CLOSE);
                    trg = true;
                    p_open = false;
                    
                } else {
                    stack.push(OTHER_FLAGS);
                    trg = true;
                    p_open = false;
                }
            }
 */

        } else if !p_open && str[iter] == '>' {

            println!("> NOT CLOSED.");
            err = true; break;
        
        } else if p_open && (str[iter].is_alphanumeric() || str[iter].is_whitespace() ||
                        str[iter] == '/' || str[iter] == '=' || str[iter] == '\"' || str[iter] == '_') {

            // Excption <Div>
            if str[iter] == '=' {
                result.push_str("title : ");

            } else if !c_open && str[iter] == '\"' {
                c_open = true;

            } else if c_open && str[iter] == '\"' {
                result.push('\n');
                c_open = false;

            } else if c_open && (str[iter].is_alphanumeric() || str[iter].is_whitespace() || str[iter] == '_') {
                result.push(str[iter]);

            // Else
            } else {
                temp.push(str[iter].to_ascii_lowercase());
            }
        
        } else if !p_open && html && div && p && str[iter] != '>' {

            if str[iter].is_whitespace() && !start_p && !is_more_whitespace {
                result.push(str[iter]);
                is_more_whitespace = true;
                end_p = false;

            } else if str[iter].is_whitespace() && ( start_p || end_p || is_more_whitespace ){
            
            } else {
                result.push(str[iter]);
                start_p = false;
                end_p = true;
                is_more_whitespace = false;
            }
        }

        // HTML TAG

        if !stack.is_empty() && trg == true {
            match stack[stack.len()-1] {

                OPEN_HTML => html = true,

                CLOSE_HTML => {
                    stack.pop(); stack.pop(); html = false;
                    if !stack.is_empty() {
                        println!("CAN NOT CLOSE HTML."); err = true;
                        break;
                    }
                }

                DIV_NOT_OPEN => {
                    println!("No Attribute div tags"); err = true; break;
                }
                DIV_OPEN => {
                    div = true;
                }
                DIV_CLOSE => {
                    stack.pop(); stack.pop(); div = false;
                }
                P_OPEN => {
                    start_p = true; p = true;
                }
                P_CLOSE => {
                    stack.pop(); stack.pop();
                    result.push('\n'); p = false;
                }
                B_OPEN => {
                    if !italic && !tag_open {
                        bold = true;
                    } else { println!("Not Closed Another Flags"); err = true; break; }
                }
                B_CLOSE => {
                    if bold {
                        stack.pop(); stack.pop(); bold = false;
                    } else { println!("Not Closed Current Flags"); err = true; break; }
                }
                ITALIC_OPEN => {
                    if !bold && !tag_open {
                        italic = true;
                    } else { println!("Not Closed Another Flags"); err = true; break; }
                }
                ITALIC_CLOSE => {
                    if italic {
                        stack.pop(); stack.pop(); italic = false;
                    } else { println!("Not Closed Current Flags"); err = true; break; }
                }
                BR_FLAGS => {
                    stack.pop();
                } /*
                OTHER_FLAGS => {
                    
                }
                OTHER_FLAGS_CLOSE => {
                    if stack[stack.len()-2] == OTHER_FLAGS {
                        println!("Not Closed Current Flags"); err = true; break;
                    } else {
                        stack.pop(); stack.pop();
                    }
                } */
                _ => { println!("Unknown tags"); err = true; break; }
                
            } trg = false;
        }

        iter += 1;
    }

    if !err {
        println!("{}", result);
    }
}

 