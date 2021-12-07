use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input_day7")?;
    let line = BufReader::new(file).lines().next();
    let numbers: Vec<i32> = line.unwrap().unwrap().split(",").map(|n| n.parse::<i32>().unwrap()).collect();
    let min = numbers.clone().into_iter().min().unwrap();
    let max = numbers.clone().into_iter().max().unwrap();
    println!("Min: {} Max: {}",min,max);
    
    let mut least_fuel = i32::MAX;
    let mut best_pos = 0;

    let mut least_fuel_2 = i32::MAX;
    let mut best_pos_2 = 0;

    for i in min..max+1{
        let mut fuel_sum = 0;
        let mut fuel_sum_2 = 0;
        for number in numbers.clone(){
            let diff = i32::abs(number-i);
            fuel_sum += diff;
            fuel_sum_2 += (diff*(diff+1))/2;
        }
        if fuel_sum < least_fuel{
            least_fuel = fuel_sum;
            best_pos = i;
        }
        if fuel_sum_2 < least_fuel_2{
            least_fuel_2 = fuel_sum_2;
            best_pos_2 = i;
        }
    }
    println!("Best pos: {}",best_pos);
    println!("Fuel sum: {}",least_fuel);

    println!("Best pos2: {}",best_pos_2);
    println!("Fuel sum2: {}",least_fuel_2);
    Ok(())
}
