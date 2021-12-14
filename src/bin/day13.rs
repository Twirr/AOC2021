use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input_day13")?;
    let mut x_max = 0;
    let mut y_max = 0;
    let mut dots = Vec::new();
    let mut folds = Vec::new();
    BufReader::new(file).lines().for_each(|line|{
        let string = line.unwrap();
        if string.contains(","){
            let mut iter = string.split(",").map(|number| number.parse::<u32>().unwrap());
            let x = iter.next().unwrap();
            let y = iter.next().unwrap();
             if x > x_max{
                x_max = x;
            }
             if y > y_max{
                y_max = y;
            }
            dots.push((x,y))
        }else if string.contains("="){
            let mut iter = string.split("=");
            folds.push((iter.next().unwrap().ends_with("x"),iter.next().unwrap().parse::<u32>().unwrap()));
        }
    });
    let mut sheet: Vec<Vec<u32>> = empty_sheet((x_max+1) as usize, (y_max+1) as usize);
    for dot in dots{
        sheet[dot.1 as usize][dot.0 as usize] += 1;
    }
    let mut first = true;
    for fold in folds{
        if fold.0{
            sheet = fold_x(sheet,fold.1 as usize);
        }else{
            sheet = fold_y(sheet,fold.1 as usize);
        }
        let visible_dots = sheet.clone().into_iter().flatten().filter(|cell| cell > &0).count();
        if first{
            println!("Result1: {}",visible_dots);
            first = false;
        }
    }
    for line in sheet{
        println!("");
        line.clone().into_iter().for_each(|n| {
            if n > 0{
                print!("#");
            }else{
                print!(" ");
            }
        });
    }

    Ok(())
}
fn fold_x(sheet: Vec<Vec<u32>>, fold_at: usize) -> Vec<Vec<u32>>{
    let old_max = fold_at*2;
    let mut new_sheet = empty_sheet(fold_at, sheet.len());
    for y in 0..sheet.len(){
        for x in 0..sheet[0].len(){
            if x < fold_at{
                new_sheet[y][x] += sheet[y][x];
            }else if x > fold_at{
                new_sheet[y][old_max-x] += sheet[y][x];
            }
        }
    }
    new_sheet
}
fn fold_y(sheet: Vec<Vec<u32>>, fold_at: usize) -> Vec<Vec<u32>>{
    let old_max = fold_at*2;
    let mut new_sheet = empty_sheet(sheet[0].len(), fold_at);
    for y in 0..sheet.len(){
        for x in 0..sheet[0].len(){
            if y < fold_at{
                new_sheet[y][x] += sheet[y][x];
            }else if y > fold_at{
                new_sheet[old_max-y][x] +=  sheet[y][x];
            }
        }
    }
    new_sheet
}
fn empty_sheet(x_max: usize, y_max: usize) -> Vec<Vec<u32>>{
    let mut sheet: Vec<Vec<u32>> = Vec::new();
    for _ in 0..y_max{
        sheet.push(vec![0; x_max as usize])
    }
    sheet
}