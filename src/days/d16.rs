use std::{cmp::Ordering, collections::{BinaryHeap, HashSet, HashMap}};

use crate::utils::utils::{get_input_file, Direction, Grid, Point};

pub fn day16_main(_second_part: bool, _extra_args: &Vec<String>){
    let input_string = get_input_file("d16.txt");
    let grid = Grid::new_from_str(&input_string);
    
    part1(&grid);
    part2(&grid);


}

#[derive(PartialEq, Eq, Debug)]
struct WeightedPoint{
    score: i64,
    direction: Direction,
    point: Point,
}

impl Ord for WeightedPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.score.cmp(&other.score){
            Ordering::Equal => self.point.cmp(&other.point),
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
        }
    }
}


impl PartialOrd for WeightedPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(grid: &Grid<char>){
    let start = grid.find_first_occurrence(&'S').expect("Contains start");

    let mut next_points: BinaryHeap<WeightedPoint> = BinaryHeap::new();
    let mut visited: HashSet<(Direction, Point)> = HashSet::new();
    next_points.push(WeightedPoint {score: 0, direction: Direction::Right, point: start});
    let mut best_score = 0;
    println!("{}", best_score); // Warning without this? rust bug?

    loop {
        let Some(p) = next_points.pop() else{panic!("We've ran out of points without reaching the end!")};
        if visited.contains(&(p.direction, p.point.clone())){continue;}
        if *grid.at_point(&p.point) == '#' {continue;}
        if *grid.at_point(&p.point) == 'E' {best_score = p.score; break;}
        visited.insert((p.direction, p.point.clone()));
        next_points.push(WeightedPoint{score: p.score + 1, direction: p.direction, point: p.point.step_direction(p.direction, grid.width(), grid.height()).expect("Inbounds")});
        next_points.push(WeightedPoint{score: p.score + 1000, direction: p.direction.turn_90_clockwise(), point: p.point.clone()});
        next_points.push(WeightedPoint{score: p.score + 1000, direction: p.direction.turn_90_anticlockwise(), point: p.point});
    }

    println!("Answer to part 1: {}", best_score);
}


fn part2(grid: &Grid<char>){
    let start = grid.find_first_occurrence(&'S').expect("Contains start");

    let mut next_points: BinaryHeap<WeightedPoint> = BinaryHeap::new();
    let mut visited: HashMap<(Point, Direction), i64> = HashMap::new();
    next_points.push(WeightedPoint{score: 0, direction: Direction::Right, point: start.clone()});
    let mut finish_score:Option<i64> = None;
    let mut final_states: Vec<(Point, Direction)> = Vec::new();

    loop {
        let Some(p) = next_points.pop() else{panic!("We've ran out of points without reaching the end!")};
        let score = p.score;
        let this_point = p.point;
        let this_dir = p.direction;

        if *grid.at_point(&this_point) == '#' {continue;}
        
        if let Some(_) = finish_score{
            if *grid.at_point(&this_point) != 'E' {break;}
        }
        
        if !visited.contains_key(&(this_point.clone(), this_dir)){
            visited.insert((this_point.clone(), this_dir), score);
        }else{
            continue;
        }

        if *grid.at_point(&this_point) == 'E' {
            finish_score = Some(score);
            final_states.push((this_point, this_dir));
            continue; // No need to generate points from the finished state
        }
        
        // Walk straight
        next_points.push(WeightedPoint{
            score: score + 1,
            point: this_point.step_direction(this_dir, grid.width(), grid.height()).expect("Inbounds"),
            direction: this_dir}
         );
        next_points.push(WeightedPoint{
            score: score + 1000,
            point: this_point.clone(),
            direction: this_dir.turn_90_clockwise(),
         
        });
        next_points.push(WeightedPoint{
            score: score + 1000,
            point: this_point.clone(),
            direction: this_dir.turn_90_anticlockwise(),
        });
    }

    let mut to_back_track: Vec<(Point, Direction)> = final_states;
    let mut visited_points: HashSet<Point> = HashSet::new();
    while let Some((p, dir)) = to_back_track.pop() {
        //println!("Backtracking: {} {:?}", p, dir);
        visited_points.insert(p.clone());
        if p == start{continue;}
        let our_score = visited.get(&(p.clone(), dir)).expect("Visited");
        let potential_previous = vec![(p.clone(), dir.turn_90_clockwise(), 1000), (p.clone(), dir.turn_90_anticlockwise(), 1000), (p.step_direction(dir.turn_180(), grid.width(), grid.height()).expect("Inbounds"), dir, 1)];
        let mut found_prev = false;
        for (prev_point, prev_dir, score_diff) in potential_previous{
            if let Some(prev_score) = visited.get(&(prev_point.clone(), prev_dir)){
                if prev_score + score_diff == *our_score{
                    to_back_track.push((prev_point, prev_dir));
                    found_prev = true;
                }
            }
        }
        assert!(found_prev);
    }


    println!("Answer to part 2: {:?}", visited_points.len());
}