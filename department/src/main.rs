use std::collections::HashMap;
use std::io;

fn main() {
    let mut company = Company {
        departments: HashMap::new(),
    };

    loop {
        let input = read_input(String::from("Please input command"));

        match input.trim() {
            "add" => add(&mut company),
            "list" => list(&company),
            _ => println!("Invali input, must match 'add'"),
        }
    }
}

fn list(company: &Company){
    let department = read_input(String::from("Please input department"));
    let employees = company.departments.get(&department);
    match employees {
        Some(vec) => println!("Employees: {:?}", vec),
        None => println!("Department does not exist")
    }
}

fn add(company: &mut Company) {
    let department = read_input(String::from("Please input department"));
    let name = read_input(String::from("Please input name"));

    company.add(&department, name);

    println!("{:?}", company);
}

fn read_input(output: String) -> String {
    println!("{output}");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

#[derive(Debug)]
struct Company {
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    fn add(&mut self, department: &str, name: String) {
        self.departments.entry(department.to_string()).or_insert(Vec::new()).push(name);
    }
}
