use std::collections::HashSet;
use crate::utils::utils::{get_input_file, Direction, Grid, Point};

pub fn day10_main(_second_part: bool, _extra_args: &Vec<String>){
    let input_str = get_input_file("d10.txt");
    
    let grid = Grid::new_from_str(&input_str);
    let mut count = 0;
    for y in 0..grid.height(){
        for x in 0..grid.width(){
            if *grid.get(x, y) != '0' {continue;}
            count += dfs_part1(&grid, &Point::new(x, y), &mut HashSet::new()) 
        }
    }
    println!("Answer to part 1: {}", count);
    
    // Part 2
    let mut count2 = 0;
    for y in 0..grid.height(){
        for x in 0..grid.width(){
            if *grid.get(x, y) != '0' {continue;}
            count2 += dfs_part2(&grid, &Point::new(x, y)) 
        }
    }
    println!("Answer to part 2: {}", count2);
}

fn acceptable_move(from: char, to: char)->bool{
    (to as i8)-(from as i8) == 1
}

fn dfs_part1(grid: &Grid<char>, point: &Point, visited: &mut HashSet<Point>)->usize{
    if visited.contains(point) {return 0;}
    visited.insert(point.clone());

    let current_val: char = *grid.at_point(point);
    if current_val == '9'{return 1}

    let mut count = 0;
    for d in vec![Direction::Left, Direction::Right, Direction::Up, Direction::Down]{
        if let Some(p) = point.step_direction(d, grid.width(), grid.height()){
            if acceptable_move(current_val, *grid.at_point(&p)){
                count += dfs_part1(grid, &p, visited);
            }
        }
    }

    count
}

fn dfs_part2(grid: &Grid<char>, point: &Point)->usize{
    let current_val: char = *grid.at_point(point);
    if current_val == '9'{return 1}

    let mut count = 0;
    for d in vec![Direction::Left, Direction::Right, Direction::Up, Direction::Down]{
        if let Some(p) = point.step_direction(d, grid.width(), grid.height()){
            if acceptable_move(current_val, *grid.at_point(&p)){
                count += dfs_part2(grid, &p);
            }
        }
    }

    count
}
