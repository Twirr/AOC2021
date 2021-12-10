use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::cmp;
fn main() -> io::Result<()> {
    let file = File::open("input_day5")?;
    let lines: Vec<String> = BufReader::new(file).lines().map(|line| line.unwrap()).collect();
    let input: Vec<(i32,i32,i32,i32)> =  lines.into_iter().map(|line|{
        let mut left_right = line.split(" -> ");

        let mut left = left_right.next().unwrap().split(',');
        let x1 = left.next().unwrap().parse::<i32>().unwrap();
        let y1 = left.next().unwrap().parse::<i32>().unwrap();
        
        let mut right = left_right.next().unwrap().split(',');
        let x2 = right.next().unwrap().parse::<i32>().unwrap();
        let y2 = right.next().unwrap().parse::<i32>().unwrap();

        (x1,y1,x2,y2)
    }).collect();
    
    let mut matrix = fill_matrix(input.clone(), false);
    let mut sum = score(matrix);
    println!("Result1: {}",sum);

    matrix = fill_matrix(input.clone(), true);
    sum = score(matrix);
    println!("Result2: {}",sum);

    Ok(())
}

fn fill_matrix(input: Vec<(i32,i32,i32,i32)>, count_diag: bool) -> Vec<Vec<i32>>{
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    for _ in 0..1000{
        matrix.push(vec![0;1000])
    }
    for t in input{
        if t.0 == t.2 {
            for i in cmp::min(t.1,t.3)..(cmp::max(t.1,t.3)+1){
                matrix[t.0 as usize][i as usize] += 1;
            }
            continue;
        }
        if t.1 == t.3{
            for i in cmp::min(t.0,t.2)..(cmp::max(t.0,t.2)+1){
                matrix[i as usize][t.3 as usize] += 1;
            }
            continue;
        }
        if count_diag{
            let steps = i32::abs(t.0-t.2)+1;
            let mut x_step = -1;
            if t.0 < t.2{
                x_step = 1;
            }
            let mut y_step = -1;
            if t.1 < t.3{
                y_step = 1;
            }
            for i in 0..steps{
                matrix[(t.0+i*x_step) as usize][(t.1+i*y_step) as usize] += 1;
            }
        }
    }
    matrix
}

fn score(matrix:  Vec<Vec<i32>>) -> i32{
    let mut sum =0;
    for i in 0..1000{
        for j in 0..1000{
            if matrix[i][j] > 1{
                sum+=1;
            }
        }
    }
    sum
}