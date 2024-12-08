use std::collections::{HashMap, HashSet};

use crate::utils::utils::{get_input_file, Grid, Point};


pub fn day8_main(_second_part: bool, _extra_args: &Vec<String>){
    let input_str = get_input_file("d8.txt");
    let grid = Grid::new_from_str(&input_str);

    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    for y in 0..grid.height(){
        for x in 0..grid.width(){
            let v = *grid.get(x, y);
            if v == '.' {continue;}
            antennas.entry(v).and_modify(|l|{l.push(Point::new(x, y));}).or_insert(vec![Point::new(x, y)]);
        }
    }

    // Part 1
    let mut antinodes: HashSet<Point> = HashSet::new();
    for (_, locations) in antennas.iter(){
        for i in 0..locations.len(){
            for j in (i+1)..locations.len(){
                let (d_x, d_y) = locations[i].delta(&locations[j]);
                if let Some(antinode_a) = locations[i].move_by(d_x, d_y, grid.width(), grid.height()){
                    antinodes.insert(antinode_a);
                }
                if let Some(antinode_b) = locations[j].move_by(-d_x, -d_y, grid.width(), grid.height()){
                    antinodes.insert(antinode_b);
                }
            }
        }
    }

    println!("Answer to part 1: {}", antinodes.len());
    
    // Part 2
    let mut antinodes_b: HashSet<Point> = HashSet::new();
    for (_, locations) in antennas.iter(){
        for i in 0..locations.len(){
            antinodes_b.insert(locations[i].clone());
            for j in (i+1)..locations.len(){
                let (d_x, d_y) = locations[i].delta(&locations[j]);
                
                let mut positive_d = locations[i].clone();
                while let Some(new_d) = positive_d.move_by(d_x, d_y, grid.width(), grid.height()){
                    antinodes_b.insert(new_d.clone());
                    positive_d = new_d;
                } 

                let mut negative_d = locations[i].clone();
                while let Some(new_d) = negative_d.move_by(-d_x, -d_y, grid.width(), grid.height()){
                    antinodes_b.insert(new_d.clone());
                    negative_d = new_d;
                } 
           }
        }
    }
    println!("Answer to part 2: {}", antinodes_b.len());
    
}