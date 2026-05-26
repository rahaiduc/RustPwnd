use std::io::{self, Write};
use std::time::Instant;
use std::collections::HashMap;

fn main() {
    //  Time execution
    let now = Instant::now();

    //  Median and mode
    let lista = vec![5,1,4,5,5,21,2,3,2,1,32,3];
    let (median, mode) = median_and_mode(lista);
    println!("The median is {median} and the mode is {mode}");
    //  Pig Latin
    let result = pig_latin("first".to_string());
    println!("{result}");
    //  Add Employee
    add_employee();

    let elapsed_time = now.elapsed();
    println!("Running slow_function() took {} seconds.", elapsed_time.as_secs_f64());

}

// Given a list of integers, use a vector and return 
// the median (when sorted, the value in the middle position) 
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.
fn median_and_mode(mut lista: Vec<i32>)->(i32, i32){
    let mut mapa: HashMap<i32, i32> = HashMap::new();
    lista.sort();
    for l in &lista{
        let count = mapa.entry(*l).or_insert(0);
        *count+=1;
    }
    let median = lista[lista.len()/2];
    let mut mode = 0;
    for (key,value) in &mapa{
        if mapa.get(&mode).copied().unwrap_or(0) < *value{
            mode=*key;
        }
    }
    (median,mode)
}

// Convert strings to Pig Latin. 
// The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. 
// Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). 
// Keep in mind the details about UTF-8 encoding!
fn pig_latin(mut word: String)->String{
    match word.chars().next(){
        Some(c)=>match c.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' =>{
                word.push_str("hay");
            },
            _ => {
                word.remove(0);
                word.push_str(&format!("{}ay",c).to_string());
            },
        },
        None=> print!("No caracter"),
    }
    return word;
}

// Using a hash map and vectors, 
// create a text interface to allow a user to add employee names to a department in a company; 
// for example, “Add Sally to Engineering” or “Add Amir to Sales.” 
// Then, let the user retrieve a list of all people in a department or 
// all people in the company by department, sorted alphabetically.
fn add_employee(){
    let mut request = String::new();
    let mut company: HashMap<String,Vec<String>> = HashMap::new();
    print!("Enter your request (Type exit for finishing): ");
    request = read_line(request);
    while request != "exit"{
        if !request.is_empty(){
            let slices: Vec<&str>= request.split(' ').collect();
            if slices.len() >= 4 && !slices[1].is_empty() && !slices[3].is_empty() {
                let name: String = slices[1].to_string();
                let department: String = slices[3].to_string();
                let employees = company
                .entry(department)
                .or_insert_with(Vec::new);

                employees.push(name);
                employees.sort();
            }
        }
        println!("View employees: Department-> Employees of the department
                                All-> All employees of the company
                                Next-> Next request
                                Exit-> Exit");
        request = read_line(request);
        match request.as_str() {
            "all"=>{
                for (departament, employee) in &company {
                    println!("Departament: {} → {:?}", departament, employee);
                }
            },
            "next"=>{},
            "exit"=>{
                break;
            }
            _=>{
                if let Some(value) = company.get(&request){
                    println!("Employees of the departament {request}: {:?}",value);
                }else{
                    println!("Department not found");
                }
            },
        }

        print!("Enter your request (Type exit for finishing): ");
        request = read_line(request);
    }
}

fn read_line(mut request: String)-> String{
    io::stdout().flush().expect("Failed to flush");
    request.clear(); io::stdin().read_line(&mut request).expect("Did not enter a correct String");
    request = request.trim().to_lowercase(); 
    return request;
}