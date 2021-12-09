use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::HashMap;
use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() -> io::Result<()> {
    let file = File::open("input_day8")?;
    let data: Vec<(Vec<String>,Vec<String>)> = BufReader::new(file).lines().map(|line|{
        let string = line.unwrap();
        let mut split = string.split("|");
        let input = split.next().unwrap().split_whitespace().map(|s|String::from(s)).collect();
        let output = split.next().unwrap().split_whitespace().map(|s|String::from(s)).collect();
        (input,output)
    }).collect();
    let mut sum = 0;
    for line in data.clone(){
        for seq in line.1{
            let len = seq.len();
            if len == 2 || len == 3 || len == 4 || len == 7 {
                sum+=1;
            }
        }
    }
    println!("Result1 {}",sum);
    let mut sum2 = 0;
    for line in data.clone(){
        let mut mapping: HashMap<u32,String> = HashMap::new();
        while mapping.len() < 10{
            for seq in line.0.clone(){
                let sorted_seq = sort(seq);
                let len = sorted_seq.clone().len();
                let _a = match len{
                    2 => mapping.insert(1,sorted_seq.clone()),
                    3 => mapping.insert(7,sorted_seq.clone()),
                    4 => mapping.insert(4,sorted_seq.clone()),
                    7 => mapping.insert(8,sorted_seq.clone()),
                    5 =>{
                        if mapping.contains_key(&1) && contains_chars(sorted_seq.clone(),mapping.get(&1).unwrap().to_string()){
                            mapping.insert(3,sorted_seq.clone());
                        }
                        if mapping.contains_key(&9) && !contains_chars(mapping.get(&9).unwrap().to_string(),sorted_seq.clone()){
                            mapping.insert(2,sorted_seq.clone());
                        }
                        if mapping.contains_key(&6) && contains_chars(mapping.get(&6).unwrap().to_string(),sorted_seq.clone()) {
                            mapping.insert(5,sorted_seq.clone());
                        }
                        None
                    },
                    6 =>{
                        if mapping.contains_key(&7) && !contains_chars(sorted_seq.clone(),mapping.get(&7).unwrap().to_string()){
                            mapping.insert(6,sorted_seq.clone());
                        }
                        if mapping.contains_key(&3) && contains_chars(sorted_seq.clone(),mapping.get(&3).unwrap().to_string()){
                            mapping.insert(9,sorted_seq.clone());
                        }
                        if mapping.contains_key(&5) && !contains_chars(sorted_seq.clone(),mapping.get(&5).unwrap().to_string()){
                            mapping.insert(0,sorted_seq.clone());
                        }
                        None
                    },
                    _ =>{
                        println!("panic {:?}",sorted_seq.clone());
                        panic!();
                    }
                };
            }
        }
        let mut rev_map: HashMap<String,char> = HashMap::new();
        for k in mapping.keys(){
            rev_map.insert(mapping.get(k).unwrap().to_string(),k.to_string().chars().next().unwrap());
        }

        let mut dec_string = String::from("");
        for seq in line.1{
            let sorted_seq = sort(seq);
            dec_string.push(*rev_map.get(&sorted_seq.clone()).unwrap());
 
        }
        sum2 += dec_string.parse::<i32>().unwrap();
    }
    println!("Result2 {}",sum2);
    Ok(())
}

fn sort(input: String) -> String{
    return input.chars().sorted().collect();
}

fn contains_chars(larger_string: String, smaller_string: String) -> bool{
    let larger_set: HashSet<char> = HashSet::from_iter(larger_string.chars());
    for c in smaller_string.chars(){
        if !larger_set.contains(&c){
            return false
        }
    }
    return true;
}