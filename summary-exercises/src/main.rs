use std::time::Instant;
use std::collections::HashMap;

fn main() {
    let now = Instant::now();

    let mut lista: Vec<i32> = Vec::new();
    let mut lista2 = vec![5,1,4,5,5,21,2,3,2,1,32,3];
    lista.append(&mut lista2);
    let (median, mode) = median_and_mode(lista);
    println!("The median is {median} and the mode is {mode}");


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
        println!("{key}:{value}");
        if mapa.get(&mode).copied().unwrap_or(0) < mapa.get(key).copied().unwrap_or(0){
            mode=*key;
        }
    }
    (median,mode)
}