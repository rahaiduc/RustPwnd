use std::time::Instant;
use std::collections::HashMap;

fn main() {
    let now = Instant::now();

    let lista = vec![5,1,4,5,5,21,2,3,2,1,32,3];
    let (median, mode) = median_and_mode(lista);
    println!("The median is {median} and the mode is {mode}");

    let result = pig_latin("first".to_string());
    println!("{result}");

    let elapsed_time = now.elapsed();
    println!("Running slow_function() took {} seconds.", elapsed_time.as_secs_f64());

}

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