use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input_day11")?;
    let data:Vec<Vec<(u32,bool)>> = BufReader::new(file).lines()
    .map(|line|{
        let string = line.unwrap();
        let line_numbers = string.chars().map(|c| (String::from(c).parse().unwrap(),false)).collect();
        line_numbers
    }).collect();

    let mut part1_copy:Vec<Vec<(u32,bool)>> = data.clone();
    let mut flashes = 0;
    for _ in 0..100{
        flashes += step(&mut part1_copy);
    }
    println!("Result1: {}",flashes);

    let mut part2_copy:Vec<Vec<(u32,bool)>> = data.clone();
    let size = part2_copy.clone().iter().flatten().count();

    let mut steps = 1;
    loop{
        flashes = step(&mut part2_copy);
        if flashes == size as u32{
            println!("Result2: {}",steps);
            break;
        }
        steps += 1;
    }

    Ok(())
}

fn step(data: &mut Vec<Vec<(u32,bool)>>) -> u32{
    data.iter_mut().flatten().for_each(|(n,b)| {
        *n += 1;
        *b = false;
    });
    let mut flashes = 0;
    for i in 0..data.len(){
        for j in 0..data[0].len(){              
            flashes += flash(data, i, j);
        }
    }

    flashes
}
fn flash(data: &mut Vec<Vec<(u32,bool)>>, i: usize, j: usize) -> u32{
    if data[i][j].1 || data[i][j].0 <= 9{
        return 0;
    }
    data[i][j].0 = 0;
    data[i][j].1 = true;
    let mut flashes = 1;
    for k in 0..3{
        for l in 0..3{
            if i+k == 0 || j+l == 0 || (k == 1 && l == 1){
                continue;
            }
            let local_i = i+k-1;
            let local_j = j+l-1;
            if local_i >= data.len() || local_j >= data[i].len(){
                continue;
            }
            if !data[local_i][local_j].1{
                data[local_i][local_j].0 += 1;
                flashes += flash(data, local_i, local_j);
            }
        }
    }
    flashes
}