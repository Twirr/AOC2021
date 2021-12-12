use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::time::Instant;



fn main() -> io::Result<()> {
    
    let file = File::open("input_day12")?;
    let ways: Vec<_> = BufReader::new(file).lines().map(|line|{
        let string = line.unwrap();
        let mut split = string.split("-");
        let to = split.next().unwrap().to_string();
        let from = split.next().unwrap().to_string();
        (to,from)
    }).collect();

    println!("Result1 {}",get_paths(true,ways.clone()));

    let start = Instant::now();
    println!("Result1 {}",get_paths(false,ways.clone()));
    let elapsed = start.elapsed();

    // Debug format
    println!("Debug: {:?}", elapsed); 
    Ok(())
}

fn get_paths(visit_once: bool, ways: Vec<(String,String)>) -> u32{
    let mut paths = Vec::new();
    let mut initial_path = Vec::new();
    initial_path.push(String::from("start"));
    paths.push((initial_path,visit_once));
    let mut finished_paths = 0;
    while paths.len() > 0{
        let mut new_paths = Vec::new();
        for path in paths.clone(){
            find_continued(ways.clone(), path, &mut new_paths, &mut finished_paths);
        }
        paths = new_paths;
    }
    finished_paths
}

fn find_continued(ways: Vec<(String,String)>,path: (Vec<String>,bool),new_paths: &mut Vec<(Vec<String>,bool)>, finished_paths: &mut u32){
    let current = path.0.last().unwrap();
    let actual_visited: HashSet<_> = path.0.clone().into_iter().filter(|node| small_cave(node)).collect();
    let visited;
    if path.1 {
        visited = actual_visited.clone();
    }else{
        visited = ["start"].into_iter().map(|s|String::from(s)).collect();
    }
    for way in ways.clone(){
        if &way.0 == current && !visited.contains(&way.1){
            let mut continued_path = path.clone();
            if way.1 == "end"{
                *finished_paths +=1;
            }else{
                if actual_visited.contains(&way.1){
                    continued_path.1 = true;
                }
                continued_path.0.push(way.1);
                new_paths.push(continued_path);
            }
        }
        else if &way.1 == current && !visited.contains(&way.0){
            let mut continued_path = path.clone();
            if way.0 == "end"{
                *finished_paths +=1;
            }else{
                if actual_visited.contains(&way.0){
                    continued_path.1 = true;
                }
                continued_path.0.push(way.0);
                new_paths.push(continued_path);
            }
        }
    }
}
fn small_cave(node: &String) -> bool{
    lazy_static! {
        static ref RE: Regex = Regex::new("[a-z]+").unwrap();
    }
    RE.is_match(node)
}