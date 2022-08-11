
use std::env;
use std::fs::File;
use std::io::Read;
use std::process;
use std::error::Error;

use super::lib2;

#[derive(Debug)]
struct Args {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Args {
    fn new(query: String, filename: String, case_sensitive: bool) -> Self {
        Self { query, filename, case_sensitive}
    }
}

pub fn main() {
    let arg: Vec<String> = env::args().collect();
    let args = get_arg(&arg).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let buf = read_file(&args.filename).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let v = find_value(&buf, &args.query);
    println!("query count: {}", v);

    let result = lib2::search(&buf, &args.query);
    println!("find query line: {:?}", result);

    let result2 = lib2::sensitive_search(&buf, &args.query);
    println!("find sensitive query line: {:?}", result2);
}

fn get_arg(arg: &[String]) -> Result<Args, &'static str> {
    let mut a = arg.iter();
    a.next();
    let query = match a.next() {
        Some(arg) => arg,
        None => return Err("did not get query"),
    };

    let filename = match a.next() {
        Some(arg) => arg,
        None => return Err("did not get filename"),
    };

    // if arg.len() < 3 {
    //     return Err("args must be at least 3 arguments");
    // }
    // let query = arg[1].clone();
    // let filename = arg[2].clone();

    let case_sensitive = std::env::var("CASE_SENSITIVE").is_err();  //  CASE_SENSITIVE=1 cargo run ggg text.txt
    println!("case_sensitive: {}", case_sensitive);

    Ok(Args::new(
        query.to_string(),
        filename.to_string(),
        case_sensitive
    ))
}

fn read_file(filename: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

fn find_value(buf: &String, query: &str) -> usize {
    let s = buf.find(query).expect("cannot find query");
    s
}

pub fn search2<'a>(contents: &'a str, query: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| {line.contains(query)}).collect()
}