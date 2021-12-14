use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let file = File::open("input_day14")?;
    let mut lines = BufReader::new(file).lines();
    let input = lines.next().unwrap().unwrap();

    let mut map = HashMap::new();
    let mut pair_counter = HashMap::new();
    lines.next();
    for line in lines{
        let string = line.unwrap();
        let mut pair = string.split(" -> ");
        let left_string = pair.next().unwrap().to_string();
        let mut left=left_string.chars();
        let right = pair.next().unwrap();

        let mut left_output = left.next().unwrap().to_string();
        left_output.push_str(right);
        let mut right_output = right.to_string();
        right_output.push(left.next().unwrap());


        map.insert(left_string.clone(),vec![left_output,right_output]);
        pair_counter.insert(left_string,0 as usize);
    }

    let chars: Vec<char> = input.chars().collect();
    for i in 1..chars.len(){
        let mut pair: String = chars[i-1].to_string();
        pair.push(chars[i]);
        *pair_counter.get_mut(&pair).unwrap() += 1;
    }

    for _ in 0..10{
        pair_counter = gen(map.clone(), pair_counter);
    }
    println!("Result1: {}",count(pair_counter.clone(),chars.clone()));
    for _ in 0..30{
        pair_counter = gen(map.clone(), pair_counter);
    }
    println!("Result2: {}",count(pair_counter.clone(),chars));

    Ok(())
}
fn gen(map: HashMap<String,Vec<String>>, pair_counter: HashMap<String,usize>) -> HashMap<String,usize>{
    let mut new_pair_counter = HashMap::new();

    for e in pair_counter{
        map.get(&e.0).unwrap().iter().for_each(|o| *(new_pair_counter.entry(String::from(o)).or_insert(0)) += e.1)
    }
    new_pair_counter
}

fn count(pair_counter: HashMap<String,usize>,chars: Vec<char>) -> usize{
    let mut result = HashMap::new();
    for e in pair_counter{
        let key = e.0.chars().next().unwrap();
        *(result.entry(key).or_insert(0)) += e.1;
    }
    //Adding last char
    *(result.get_mut(chars.last().unwrap()).unwrap()) += 1;

    let max=  result.values().max().unwrap();
    let min = result.values().min().unwrap();
    max-min
}