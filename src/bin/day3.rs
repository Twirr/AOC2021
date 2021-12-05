use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input_day3")?;
    let lines = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    
    part1(lines.clone());
    part2(lines.clone());

    Ok(())
}

fn part1(lines: Vec<String>) {
    let mut status: [i32;12] = [0;12];
    for line in lines{
        let mut i = 0;
        for c in line.chars(){
            match c {
                '1' => status[i] += 1,
                '0' => status[i] -= 1,
                // Handle the rest of cases
                _ => println!("Ain't special"),
            }
            i += 1;
        }
    };
    let mut gamma = String::from("");
    let mut epsilon  = String::from("");
    for number in status.into_iter() {
        if number > 0{
            gamma.push('1');
            epsilon.push('0');
        }else{
            gamma.push('0');
            epsilon.push('1');
        }
    };
    println!("Gamma: {}",gamma);
    println!("Epsilon: {}",epsilon);

    let gamma_dec = isize::from_str_radix(&gamma, 2).unwrap();
    let epsi_dec = isize::from_str_radix(&epsilon, 2).unwrap();
    println!("Gamma dec: {}",gamma_dec);
    println!("Epsilon dec: {}",epsi_dec);

    println!("Result1 {}",gamma_dec*epsi_dec);
}


fn part2(lines: Vec<String>) {
    let (val,mut oxy) = reduce(lines.clone(),true,String::from(""));
    oxy.push_str(&val.into_iter().next().unwrap());
    let oxy_dec = isize::from_str_radix(&oxy, 2).unwrap();
    println!("Oxy: {}",oxy);
    println!("Oxy dec: {}",oxy_dec);

    let (val2,mut co2) = reduce(lines.clone(),false,String::from(""));
    co2.push_str(&val2.into_iter().next().unwrap());
    let c02_dec = isize::from_str_radix(&co2, 2).unwrap();

    println!("Co2: {}",co2);

    println!("Co2 dec: {}",c02_dec);

    println!("Result2 {}",oxy_dec*c02_dec);
}

fn reduce(lines: Vec<String>, oxy: bool, result: String) -> (Vec<String>, String){
    let mut new_result = result.clone();
    let lines2 = lines.clone();
    let mut status = 0;
    for line in lines{
        let c = line.chars().next().unwrap();
        match c {
            '1' => status += 1,
            '0' => status -= 1,
            // Handle the rest of cases
            _ => println!("Ain't special"),
        }
    }
    let keep = status_to_keep(status,oxy);
    new_result.push(keep);
    let mut reduced = lines2.into_iter().filter(|line| line.chars().next().unwrap() == keep).map(|line| String::from(&line[1..])).collect::<Vec<String>>();
    if reduced.len() > 1{
        let (tmp_red,tmp_res) = reduce(reduced.clone(),oxy,new_result);
        reduced = tmp_red;
        new_result = tmp_res;
    }
    (reduced,new_result)
}

fn status_to_keep(status: i32, oxy: bool) -> char{
    let mut keep = '0';
    if oxy && status >= 0 || !oxy && status < 0{
        keep = '1';
    }
    keep
}