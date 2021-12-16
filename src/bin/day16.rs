use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::time::Instant;

fn main() -> io::Result<()> {
    let file = File::open("input_day16")?;
    let mut data = BufReader::new(file).lines().map(|line| hex::decode(line.unwrap()).unwrap()).flatten().
                            map(|byte|format!("{:01$b}",byte,8)).collect::<Vec<String>>().join("");
    let mut next_char = 0;
    let mut version_sum = 0;

    let result2 = parse_packet(&mut data, &mut next_char, &mut version_sum);
    

    let start = Instant::now();
    println!("Result1: {}, Result2: {} Time: {:?}",version_sum,result2,start.elapsed());

    Ok(())
}
fn parse_packet(data: &mut String,next_char: &mut usize, version_sum: &mut u64) -> u64{
    *version_sum += read_int(data,next_char,3);
    let type_dec = read_int(data,next_char,3);
    return match type_dec {
        4 => parse_literal(data,next_char),
        _ => {
            let mut literals = Vec::new();

            if read_int(data, next_char, 1) == 0{
                let packet_end = read_int(data, next_char, 15) as usize + *next_char;
                while *next_char < packet_end{
                    literals.push(parse_packet(data, next_char, version_sum));
                }
            }else{
                for _ in 0..read_int(data, next_char, 11){
                    literals.push(parse_packet(data, next_char, version_sum));
                }
            }
            match type_dec{
                0 => literals.into_iter().sum(),
                1 => literals.into_iter().product(),
                2 => literals.into_iter().min().unwrap(),
                3 => literals.into_iter().max().unwrap(),
                5 => (literals.get(0).unwrap() > literals.get(1).unwrap()) as u64,
                6 => (literals.get(0).unwrap() < literals.get(1).unwrap()) as u64,
                7 => (literals.get(0).unwrap() == literals.get(1).unwrap()) as u64,
                _ => 0
            }
        },
    }
}
fn parse_literal(data: &mut String,next_char: &mut usize) -> u64{
    let mut last_literal = false;
    let mut literal = String::from("");
    while !last_literal{
        last_literal = read_int(data, next_char, 1) == 0;
        literal.push_str(&read_str(data, next_char, 4));
    }
    bin_to_dec(&literal)
}
fn read_int(data: &mut String,next_char: &mut usize, bits: usize) -> u64{
    bin_to_dec(&read_str(data, next_char, bits))
}
fn read_str(data: &mut String,next_char: &mut usize, bits: usize) -> String{
    let bin_value = &data[*next_char..*next_char+bits];
    *next_char+=bits;
    String::from(bin_value)
}

fn bin_to_dec(bin: &str) -> u64{
    return u64::from_str_radix(bin, 2).unwrap();
}