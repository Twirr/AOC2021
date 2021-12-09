use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

use itertools::Itertools;

fn main() -> io::Result<()> {
    let file = File::open("input_day9")?;
    let data: Vec<Vec<u32>> = BufReader::new(file).lines().map(|line|{
        let string = line.unwrap();
        let line_numbers = string.chars().map(|c| String::from(c).parse().unwrap()).collect();
        line_numbers
    }).collect();
    let mut sum = 0;
    let mut basin_sizes: Vec<u64> = Vec::new();
    for i in 0..data.len(){
        for j in 0..data[i].len(){
            let cell = data[i][j];
            let mut local_min = true;
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
                    if cell >= data[local_i][local_j]{
                        local_min = false;
                    }
                }
            }
            if local_min{
                sum += cell+1; //part1

                let mut search_points: Vec<(usize,usize)> = Vec::new();
                search_points.push((i,j));

                let mut visited: Vec<(usize,usize)> = Vec::new(); 
                while !search_points.is_empty(){
                    for t in search_points.clone(){
                        visited.push(t);
                    }
                    search_points = search(search_points.clone(),data.clone(),visited.clone());
                }
                //WHY DO WE HAVE DUPLICATES, AHH
                basin_sizes.push(visited.into_iter().sorted().dedup().count() as u64);
            } 
        }
    }
    println!("Result1: {}",sum);
    let mut sorted_basins = basin_sizes.into_iter().sorted().rev();
    let mut basin_prod: u64 = 1;
    for _ in 0..3{
        basin_prod *= sorted_basins.next().unwrap();
    }
    println!("Result2: {}",basin_prod);
    Ok(())
}
fn search(search_points: Vec<(usize,usize)>, data: Vec<Vec<u32>>, visited: Vec<(usize,usize)>) -> Vec<(usize,usize)>{
    let mut new_search_points: Vec<(usize,usize)> = Vec::new();

    for cell in search_points{
        let mut i = cell.0;
        let mut j = cell.1;
        while i > 0{
            i -= 1;
            if data[i][j] == 9 || visited.contains(&(i,j)){
                break;
            }
            new_search_points.push((i,j));
        }

        i = cell.0;
        j = cell.1;
        while j > 0{
            j -= 1;
            if data[i][j] == 9 || visited.contains(&(i,j)){
                break;
            }
            new_search_points.push((i,j));
        }

        i = cell.0;
        j = cell.1;
        while i < data.len()-1{
            i += 1;
            if data[i][j] == 9 || visited.contains(&(i,j)){
                break;
            }
            new_search_points.push((i,j));
        }

        i = cell.0;
        j = cell.1;
        while j < data[i].len()-1{
            j += 1;
            if data[i][j] == 9 || visited.contains(&(i,j)){
                break;
            }
            new_search_points.push((i,j));
        }
    }

    new_search_points
}