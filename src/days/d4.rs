use std::char;

use crate::utils::utils::{get_input_file, Grid, GridPoint};

fn slice_is_xmas(slice: Option<Vec<char>>)->bool{
    if let Some(v) = slice{
        let v_str:String = v.into_iter().collect();
        return v_str == "XMAS" || v_str == "SAMX"; 
    }
    false
}

pub fn part1(grid: &Grid<char>){
    let mut match_count:usize = 0;
    
    for y in 0..grid.height(){
        for x in 0..grid.width(){
            let point = grid.at(x, y).expect("In bounds");
            if slice_is_xmas(point.right_slice(4)){match_count+=1}
            if slice_is_xmas(point.down_slice(4)){match_count+=1}
            if slice_is_xmas(point.down_right_slice(4)){match_count+=1}
            if slice_is_xmas(point.down_left_slice(4)){match_count+=1}
        }
    }

    println!("Answer Part 1: {}", match_count);
}

fn point_is_xmas(point: &GridPoint<char>) -> bool{
    if *point.value() != 'A'{return false;}
    let ul = if let Some(v) = point.up_left(){v.value().clone()}else{return false;};
    let ur = if let Some(v) = point.up_right(){v.value().clone()}else{return false;};
    let dl = if let Some(v) = point.down_left(){v.value().clone()}else{return false;};
    let dr = if let Some(v) = point.down_right(){v.value().clone()}else{return false;};

    let uphill_mas = (dl=='M' && ur=='S') || (dl=='S' && ur=='M');
    let downhill_mas = (dr=='M' && ul=='S') || (dr=='S' && ul=='M');

    uphill_mas && downhill_mas
}

fn part2(grid: &Grid<char>){
    let mut match_count:usize = 0;
    
    for y in 0..grid.height(){
        for x in 0..grid.width(){
            let point = grid.at(x, y).expect("In bounds");
            if point_is_xmas(&point){match_count+=1}
        }
    }

    println!("Answer Part 2: {}", match_count);
}

pub fn day4_main(second_part: bool, _extra_args: &Vec<String>){
    let input_file_str = get_input_file("d4.txt");
    let grid = Grid::new_from_str(&input_file_str);
    
    if second_part{
        part2(&grid);
    }else{
        part1(&grid);
    }
}