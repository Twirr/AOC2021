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
    parse_version(data, next_char, version_sum);
    parse_type(data, next_char, version_sum)
}
fn parse_version(data: &mut String,next_char: &mut usize, version_sum: &mut u64){
    let ver_bin = &data[*next_char..*next_char+3];
    let ver_dec =  bin_to_dec(ver_bin);
    *version_sum += ver_dec;
    *next_char+=3;
}

fn parse_type(data: &mut String,next_char: &mut usize, version_sum: &mut u64) -> u64{
    let type_bin = &data[*next_char..*next_char+3];
    let type_dec = bin_to_dec(type_bin);
    *next_char+=3;
    return match type_dec {
        4 => parse_literal(data,next_char),
        _ => {
            let mut literals = Vec::new();

            let length_type_id = &data[*next_char..*next_char+1] == "0";
            *next_char+=1;
            if length_type_id{
                let length_bin = &data[*next_char..*next_char+15];
                *next_char+=15;
                let packet_end = *next_char+bin_to_dec(length_bin) as usize;
                while *next_char < packet_end{
                    literals.push(parse_packet(data, next_char, version_sum));
                }
            }else{
                let packet_amount_bin = &data[*next_char..*next_char+11];
                *next_char+=11;
                for _ in 0..bin_to_dec(packet_amount_bin){
                    literals.push(parse_packet(data, next_char, version_sum));
                }
            }
            match type_dec{
                0 => literals.into_iter().sum(),
                1 => literals.into_iter().product(),
                2 => literals.into_iter().min().unwrap(),
                3 => literals.into_iter().max().unwrap(),
                5 => {
                    if literals.get(0).unwrap() > literals.get(1).unwrap(){
                        1
                    }else{
                        0
                    }
                },
                6 => {
                    if literals.get(0).unwrap() < literals.get(1).unwrap(){
                        1
                    }else{
                        0
                    }
                }
                7 => {
                    if literals.get(0).unwrap() == literals.get(1).unwrap(){
                        1
                    }else{
                        0
                    }
                }
                _ => 0
            }
        },
    }
}
fn parse_literal(data: &mut String,next_char: &mut usize) -> u64{
    let mut last_literal = false;
    let mut literal = String::from("");
    while !last_literal{
        last_literal = &data[*next_char..*next_char+1] == "0";
        let part_value = &data[*next_char+1..*next_char+5];
        literal.push_str(part_value.clone());
        *next_char+=5;
    }
    bin_to_dec(&literal)
}


fn bin_to_dec(bin: &str) -> u64{
    return u64::from_str_radix(bin, 2).unwrap();
}