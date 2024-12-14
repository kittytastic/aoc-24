use std::collections::HashSet;

use crate::utils::utils::{get_input_file, Direction, Grid, Point, get_number_substr};

pub fn day14_main(_second_part: bool, _extra_args: &Vec<String>){
    let input_string = get_input_file("d14.txt");

    let input: Vec<((usize, usize), (i64, i64))> = input_string.lines().map(|l|{
        let numbers = get_number_substr(l); 
        ((numbers[0].parse().expect("Valid num"), numbers[1].parse().expect("Valid num")),
         (numbers[2].parse().expect("Valid num"), numbers[3].parse().expect("Valid num")))
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

    // Part 2
    part2(&input);
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

fn get_grid_at(input: &Vec<((usize, usize), (i64, i64))>, seconds: i64, width: usize, height: usize)->Grid<char>{
    let mut grid = Grid::new_from('.', height, width);
    for ((x, y), (vx, vy)) in input.iter().cloned(){
        let current_pos = Point::new(x, y);
        let new_pos = current_pos.move_by_modulo(vx*seconds, vy*seconds, width, height);
        grid.set(new_pos.get_x(), new_pos.get_y(), 'X');
    };
    grid
}

fn count_neighbours(input: &Vec<((usize, usize), (i64, i64))>, seconds: i64, width: usize, height: usize)->usize{
    let mut seen: HashSet<Point> = HashSet::new();
    let mut neighbour_count = 0;
    for ((x, y), (vx, vy)) in input.iter().cloned(){
        let current_pos = Point::new(x, y);
        let new_pos = current_pos.move_by_modulo(vx*seconds, vy*seconds, width, height);
        seen.insert(new_pos.clone());
        for d in vec![Direction::Up, Direction::Left, Direction::Right, Direction::Down]{
            if let Some(nn) = new_pos.step_direction(d, width, height){
                if seen.contains(&nn){
                    neighbour_count+=1
                }
            }
        }
    };
    neighbour_count
}

fn part2(input: &Vec<((usize, usize), (i64, i64))>){
    let height = 103;
    let width = 101;
    
    let mut best_time = 0;
    let mut best_count = 0;
    for i in 0..101{
        for j in 0..103{
            let t = i*103 + j;
            let c = count_neighbours(input, t, width, height);
            if c > best_count{
                best_count = c;
                best_time = t;
            }
        }
    }

    println!("{}", get_grid_at(input, best_time, width, height));
    println!("Answer to part 2: {}", best_time);
}