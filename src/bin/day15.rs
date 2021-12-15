use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::time::Instant;

fn main() -> io::Result<()> {
    let file = File::open("input_day15")?;
    let mut visited_paths:Vec<Vec<i32>> = Vec::new();
    let mut data: Vec<Vec<i32>> = BufReader::new(file).lines().map(|line|{
        let string = line.unwrap();
        let line_numbers:Vec<i32> = string.chars().map(|c| String::from(c).parse().unwrap()).collect();
        visited_paths.push(vec!(i32::MAX;line_numbers.len()));
        line_numbers
    }).collect();
    let mut start = Instant::now();
    let risk = solve(&mut data,&mut visited_paths);
    println!("Result1: {} Time: {:?}",risk,start.elapsed());

    let x_block_size = data.len();
    let y_block_size = data[0].len();
    let mut visited_paths2:Vec<Vec<i32>> = Vec::new();
    let mut data2:Vec<Vec<i32>> = Vec::new();
    for _ in 0..5*x_block_size{
        visited_paths2.push(vec!(i32::MAX;5*y_block_size));
        data2.push(vec!(0;5*y_block_size))
    }
    for x_block in 0..5{
        for y_block in 0..5{
            for x in 0..x_block_size{
                for y in 0..y_block_size{
                    let mut a = data[x][y] + x_block as i32+ y_block as i32;
                    if a > 9{
                        a -= 9;
                    }
                    data2[x_block*x_block_size+x][y_block*y_block_size+y] = a;
                }
            }
        }
    }

    start = Instant::now();
    let risk = solve(&mut data2,&mut visited_paths2);
    println!("Result2: {} Time: {:?}",risk,start.elapsed());

    Ok(())
}
fn solve(data: &mut Vec<Vec<i32>>, visited_paths: &mut Vec<Vec<i32>>) -> i32{
    let mut cells_to_visit = Vec::new();
    cells_to_visit.push((0,0,0));
    while cells_to_visit.len() > 0{
        let mut new_cells_to_visit = Vec::new();
        for c in cells_to_visit{
            visit(data, visited_paths, (c.0,c.1), c.2).into_iter().for_each(|l|new_cells_to_visit.push(l));
        }
        cells_to_visit = new_cells_to_visit;
    }
    return *visited_paths.last().unwrap().last().unwrap()-*(data.first().unwrap().first().unwrap()) //Remove cell 0,0
}
fn visit(data: &mut Vec<Vec<i32>>, visited_paths: &mut Vec<Vec<i32>>, c: (usize,usize), risk: i32) -> Vec<(usize,usize,i32)>{
    let mut cells_to_visit = Vec::new();
    if c.0 == data.len()-1 && c.1 == data[0].len(){
        return cells_to_visit;
    }
    let new_risk = risk + data[c.0][c.1] as i32;
    if new_risk < visited_paths[c.0][c.1]{
        visited_paths[c.0][c.1] = new_risk;
        if c.0 > 0{
            cells_to_visit.push((c.0-1,c.1,new_risk));
        }
        if c.1 > 0{
            cells_to_visit.push((c.0,c.1-1,new_risk));
        }
        if c.1+1 < visited_paths[0].len() {
            cells_to_visit.push((c.0,c.1+1,new_risk));
        }
        if c.0+1 < visited_paths.len(){
            cells_to_visit.push((c.0+1,c.1,new_risk));
        }
    }
    cells_to_visit
}