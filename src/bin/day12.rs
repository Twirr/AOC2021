use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::time::Instant;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let file = File::open("input_day12")?;
    let mut ways: HashMap<String,Vec<String>> = HashMap::new();
    BufReader::new(file).lines().for_each(|line|{
        let string = line.unwrap();
        let mut split = string.split("-");
        let to = split.next().unwrap();
        let from = split.next().unwrap();

        ways.entry(String::from(to)).or_insert(<Vec<String>>::new()).push(String::from(from));
        ways.entry(String::from(from)).or_insert(<Vec<String>>::new()).push(String::from(to));
    });

    println!("Result1 {}",get_paths(true,ways.clone()));

    let start = Instant::now();
    println!("Result1 {}",get_paths(false,ways.clone()));
    let elapsed = start.elapsed();

    // Debug format
    println!("Debug: {:?}", elapsed); 
    Ok(())
}

fn get_paths(visit_once: bool, ways: HashMap<String,Vec<String>>) -> u32{
    let mut paths = Vec::new();
    let mut initial_path = Vec::new();
    initial_path.push(String::from("start"));
    paths.push((initial_path,visit_once));
    let mut finished_paths = 0;
    let mut visited = HashSet::new();
    visited.insert(String::from("start"));

    for path in paths.clone(){
        find_continued(ways.clone(), path, &mut finished_paths, visited.clone());
    }

    finished_paths
}

fn find_continued(ways: HashMap<String,Vec<String>>, path: (Vec<String>,bool), finished_paths: &mut u32, actual_visited: HashSet<String>){
    let current = path.0.last().unwrap();
    let visited;
    if path.1 {
        visited = actual_visited.clone();
    }else{
        visited = [String::from("start")].into_iter().collect();
    }
    for way in ways.get(current).unwrap(){
        if !visited.contains(way){
            let mut continued_path = path.clone();
            if *way == "end"{
                *finished_paths +=1;
            }else{
                if actual_visited.contains(way){
                    continued_path.1 = true;
                }
                continued_path.0.push(String::from(way));
                let mut new_visited = actual_visited.clone(); 
                if small_cave(way){
                    new_visited.insert(String::from(way));
                }
                find_continued(ways.clone(), continued_path, finished_paths,new_visited);
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