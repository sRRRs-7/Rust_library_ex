
use std::io::{stdin, stdout, Write, Read};
use std::fs::{OpenOptions, File};
use std::env::args;
use serde::{Serialize, Deserialize};
use serde_json::json;


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
    stdin().read_line(&mut name).unwrap();      // buffer
    let _ = stdout().flush();                        // buffer refresh
    let name = name.trim().to_string();

    // enter text
    let mut department = String::new();
    print!("enter employee department: ");
    stdin().read_line(&mut department).unwrap();
    let _ = stdout().flush();
    let department = department.trim().to_string();

    // write
    write_file(name, department);
}

fn load() -> Vec<Employee> {
    let mut file = File::open("employee.json").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let v:Vec<Employee> = serde_json::from_str(&buf).unwrap();
    v
}

// read
fn read_file() {
    let emp = load();
    for e in emp.iter() {
        println!("{:?}", e);
    }
}


fn search() {
    let mut search = String::new();
    print!("enter search value: ");
    let _ = stdout().flush();
    stdin().read_line(&mut search).unwrap();
    let search = search.trim().to_string();

    let json = load();
    for js in json.into_iter() {
        let j = json!(js);
        let v = j.get(&search).unwrap();
        println!("{}", v);
    };
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

    let serialized = serde_json::to_string(&emp).unwrap();

    write!(&file, "{}", serialized).unwrap();
}

// init data
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

    let serialized = serde_json::to_string(&arr).unwrap();

    write!(&file, "{}", serialized).unwrap();
}



//

// json
// fn convert_json(name: String, department: String) -> Value {
//     let s = json!({
//         "name": name,
//         "department": department,
//     });
//     s
// }
