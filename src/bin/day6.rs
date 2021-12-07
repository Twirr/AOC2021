use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let file = File::open("input_day6")?;
    let line = BufReader::new(file).lines().next();
    let numbers: Vec<u32> = line.unwrap().unwrap().split(",").map(|n| n.parse::<u32>().unwrap()).collect();
    let mut map = HashMap::new();
    for i in 1..6{
        let sum = numbers.clone().into_iter().filter(|n| *n == i).count();
        map.insert(i,u64::try_from(sum).unwrap());
    }
    
    for gen in 0..256{
        let mut new_map = HashMap::new();
        for t in map{
            if t.0 == 0{
                new_map.insert(8,t.1);
                let sum_opt = new_map.get(&6);
                let mut sum = 0;
                if sum_opt.is_some(){
                    sum = *sum_opt.unwrap();
                }
                new_map.insert(6,sum+t.1);
            }else{
                let sum_opt = new_map.get(&(t.0-1));
                let mut sum = 0;
                if sum_opt.is_some(){
                    sum = *sum_opt.unwrap();
                }
                new_map.insert(t.0-1,sum+t.1);
            }
        } 
        map = new_map;
        if gen == 79{
            let fishes = map.clone().into_iter().map(|t| t.1).sum::<u64>();
            println!("Fishes gen 80: {}",fishes);
        }
    }
    let fishes = map.into_iter().map(|t| t.1).sum::<u64>();
    println!("Fishes gen 256: {}",fishes);
    Ok(())
}
