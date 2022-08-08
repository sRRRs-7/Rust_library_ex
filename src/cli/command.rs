
use std::env;
use std::fs::File;
use std::io::Read;
use std::process;
use std::error::Error;



struct Args {
    query: String,
    filename: String,
}

impl Args {
    fn new(query: String, filename: String) -> Self {
        Self { query, filename}
    }
}

pub fn main() {
    let arg: Vec<String> = env::args().collect();
    let args = get_arg(&arg).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let buf = read_file(&args.filename).unwrap();

    let v = find_value(&buf, &args.query);
    println!("query count: {}", v);

    let result = search(&buf, &args.query);
    println!("find query line: {:?}", result);
}

fn get_arg(arg: &[String]) -> Result<Args, &'static str> {
    if arg.len() < 3 {
        return Err("args must be at least 3 arguments");
    }
    let query = arg[1].clone();
    let filename = arg[2].clone();

    Ok(Args::new(query, filename))
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

pub fn search<'a>(contents: &'a str, query: &'a str) -> Vec<&'a str> {
    let mut r = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            r.push(line.trim())
        }
    }
    r
}