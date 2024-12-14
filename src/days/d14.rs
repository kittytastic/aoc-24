use std::str::FromStr;

use crate::utils::utils::{get_input_file, Point};

pub fn day14_main(_second_part: bool, _extra_args: &Vec<String>){
    let input_string = get_input_file("d14.txt");
    let input:Vec<((usize, usize), (i64, i64))> = input_string.lines().map(|l|{
        let parts:Vec<&str> = l.split_ascii_whitespace().collect();
        (get_pair::<usize>(parts[0]), get_pair::<i64>(parts[1]))
    }).collect();

    let seconds:i64 = 100;
    let height = 103;
    let width = 101;
    let mut quadrent_count = vec![0,0,0,0];
    for ((x, y), (vx, vy)) in input.iter(){
        let current_pos = Point::new(*x, *y);
        let new_pos = current_pos.move_by_modulo((*vx)*seconds, (*vy)*seconds, width, height);
        match place_in_quadrant(&new_pos, width, height){
            Quadrent::TopLeft => quadrent_count[0] +=1,
            Quadrent::TopRight => quadrent_count[1] +=1,
            Quadrent::BottomLeft => quadrent_count[2] +=1,
            Quadrent::BottomRight => quadrent_count[3] +=1,
            Quadrent::None => (),
        }
    }
    let saftey_factor = quadrent_count.iter().cloned().reduce(|acc, e| acc*e).expect("value");
    println!("Answer to part 1: {}", saftey_factor);
}

fn get_pair<T:FromStr>(s:&str)->(T, T){
    let ignore = &['=', 'p', 'v'];
    let s = s.trim_matches(ignore);
    let parts:Vec<&str> = s.split(',').collect();
    (match parts[0].parse() {Ok(v) => v, Err(_) => panic!("p0 failed")}, match parts[1].parse() {Ok(v) => v, Err(_) => panic!("p1 failed")})
}

enum Quadrent{
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    None
}
fn place_in_quadrant(point: &Point, width: usize, height: usize)->Quadrent{
    let mid_x = width/2;
    let mid_y = height/2;
    
    if point.get_x() < mid_x {
        if point.get_y()<mid_y{
            Quadrent::TopLeft
        }else if point.get_y()>mid_y {
            Quadrent::BottomLeft
        }else{
            Quadrent::None
        }
    }else if point.get_x() > mid_x {
        if point.get_y()<mid_y{
            Quadrent::TopRight
        }else if point.get_y()>mid_y {
            Quadrent::BottomRight
        }else{
            Quadrent::None
        }
    }else{
        Quadrent::None
    }
}