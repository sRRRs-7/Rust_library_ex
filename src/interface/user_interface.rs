
use std::io::{stdin, stdout, Write, Read};
use std::fs::{OpenOptions, File};
use std::env::args;
use serde::ser::SerializeMap;
use serde::{Serialize, Deserialize};
use serde_json::{json};
use std::io::BufReader;
use serde_json::Value;
extern crate rustc_serialize;


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Employee {
    name: String,
    department: String,
}

pub fn main() {
    let args: Vec<_> = args().collect();
    let arg: String = args[1].parse().unwrap();
    match &arg[..] {
        "write" => input_json(),
        "add" => add(),
        "read" => {
            let j = read_file();
            println!("{:?}", j);
        },
        "search" => search(),
        _ => (),
    }
}

// input json
fn input_json() {
    // enter text
    let mut name = String::new();
    print!("enter employee name: ");
    let _ = stdout().flush();
    stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();

    // enter text
    let mut department = String::new();
    print!("enter employee department: ");
    let _ = stdout().flush();
    stdin().read_line(&mut department).unwrap();
    let department = department.trim().to_string();

    // write
    write_file(name, department);
}

fn search() {
    let json = read_file();
    println!("{:?}", json);
}

// read
fn read_file() -> Employee {
    let file = OpenOptions::new()
        // .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("employee.json")
        .unwrap();
    let reader = BufReader::new(file);
    let emp: Employee = serde_json::from_reader(reader).unwrap();
    println!("{:?}", emp);
    emp
}

fn load() -> Vec<Employee> {
    let mut file = File::open("employee.json").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let v:Vec<Employee> = serde_json::from_str(&buf).unwrap();
    println!("{:?}", v);
    v
}

// write
fn write_file(name: String, department: String) {
    let file = OpenOptions::new()
        // .read(true)
        // .append(true)
        .write(true)
        .create(true)
        .open("employee.json")
        .unwrap();

    // read file
    let mut emp = load();
    // new data
    let e = Employee{name, department};
    emp.push(e);

    let serialized = serialize_json(emp);

    write!(&file, "{}", serialized).unwrap();
}

fn serialize_json(arr: Vec<Employee>) -> String {
    let serialized: String = serde_json::to_string(&arr).unwrap();
    serialized
}

fn add() {
    let file = OpenOptions::new()
    // .read(true)
      // .append(true)
    .write(true)
    .create(true)
    .open("employee.json")
    .unwrap();

    let name = String::from("name");
    let department = String::from("department");
    let emp = Employee{
        name,
        department,
    };

    let mut arr = Vec::new();
    arr.push(emp);

    let serialized = serialize_json(arr);

    write!(&file, "{}", serialized).unwrap();
}


// json
// fn convert_json(name: String, department: String) -> Value {
//     let s = json!({
//         "name": name,
//         "department": department,
//     });
//     s
// }
