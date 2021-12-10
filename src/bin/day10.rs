use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

use std::collections::HashMap;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let file = File::open("input_day10")?;
    let lines: Vec<String> = BufReader::new(file).lines().map(|line| line.unwrap()).collect();
    let mut map: HashMap<char,char> = HashMap::new();
    map.insert('(',')');
    map.insert('{','}');
    map.insert('<','>');
    map.insert('[',']');

    let mut score_map: HashMap<char,u64> = HashMap::new();
    score_map.insert(')',3);
    score_map.insert(']',57);
    score_map.insert('}',1197);
    score_map.insert('>',25137);

    let mut score = 0;
    for line in lines.clone(){
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars(){
            if map.contains_key(&c){
                stack.push(c)
            }else if stack.is_empty() || c != *map.get(stack.last().unwrap()).unwrap(){
                score += score_map.get(&c).unwrap();
                break;
            }else{
                stack.pop();
            }
        }
    }
    println!("Result1: {}",score);

    let mut score_map2: HashMap<char,u64> = HashMap::new();
    score_map2.insert(')',1);
    score_map2.insert(']',2);
    score_map2.insert('}',3);
    score_map2.insert('>',4);

    let mut scores: Vec<u64> = Vec::new();
    for line in lines.clone(){
        let mut score = 0u64;
        let mut stack: Vec<char> = Vec::new();
        let mut corrupt = false;
        for c in line.chars(){
            if map.contains_key(&c){
                stack.push(c)
            }else if stack.is_empty() || c != *map.get(stack.last().unwrap()).unwrap(){
                corrupt = true;
                break;
            }else{
                stack.pop();
            }
        }
        if !corrupt{
            stack.clone().into_iter().rev().for_each(|c|{
                score = 5*score + score_map2.get(map.get(&c).unwrap()).unwrap();
            });
            scores.push(score);
        }
    }
    let sorted_scores: Vec<u64> = scores.into_iter().sorted().collect();
    let middle_score = sorted_scores.get((sorted_scores.len()-1)/2);
    
    println!("Result2: {}",middle_score.unwrap());
    Ok(())
}