use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// TODO 当然コマンドラインからオプションで指定できるように。
const DOCUMENT_ROOT: &str = "/home/koji/work/html/";

pub fn read_file(file_name: &str) -> String {

    let chars = file_name.chars().collect::<Vec<char>>();
    let mut file_name = String::new();
    for c in &chars[1..] {
        file_name.push(*c);
    }

    if file_name == "" {
        file_name = "index.html".to_string();
    }

    let path = &format!("{}{}", DOCUMENT_ROOT, file_name);
    let path = Path::new(path);
    let mut file = match File::open(&path) {
        Err(e) => {
            panic!(
                "could not open{}: {}",
                path.display(),
                Error::description(&e)
            )
        }
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(e) => {
            panic!(
                "could not read {}: {}",
                path.display(),
                Error::description(&e)
            )
        }
        Ok(_) => (),
    }
    s
}
fn main() {
    //println!("{}", read_file("/index.html"));
    let s = "aiueo";
    let t = {
        let mut s = self.chars();
        s.next();
        s.as_str()
    };
    println!("{}", t);

}
