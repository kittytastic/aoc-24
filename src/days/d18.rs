use std::{cmp::Ordering, collections::{BinaryHeap, HashSet}};


use crate::utils::utils::{get_input_file, Direction, Grid, Point};

pub fn day18_main(_second_part: bool, _extra_args: &Vec<String>){
    let input_string = get_input_file("d18.txt");
    let height = 71;
    let width = 71;
    let barrier_start = 1024;

    let all_barriers: Vec<Point> = input_string.lines().map(|l|{
        let s = l.split(',').collect::<Vec<_>>();
        Point::new(s[0].parse().expect("Valid number"), s[1].parse().expect("Valid number"))
    }).collect();

    let p1_barriers:HashSet<Point> = all_barriers.iter().take(barrier_start).cloned().collect();
    let p1_answer = traverse_maze(width, height, &p1_barriers).expect("Should find path");
    println!("Answer to part 1: {}", p1_answer);

    let mut p2_barriers: HashSet<Point> = HashSet::new();
    for b in all_barriers.iter(){
        p2_barriers.insert(b.clone());
        let maze_result = traverse_maze(width, height, &p2_barriers);
        if maze_result.is_none(){println!("Answer to part 2: {}", b); break;}
    }

}

#[derive(PartialEq, Eq, Debug)]
struct MinHeapContainer<T, S>{
    weight: T,
    value: S
}

impl<T: Ord, S: Ord> Ord for MinHeapContainer<T, S> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.weight.cmp(&other.weight){
            Ordering::Equal => self.value.cmp(&other.value),
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
        }
    }
}


impl<T: Ord, S: Ord> PartialOrd for MinHeapContainer<T, S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn traverse_maze(width: usize, height: usize, barriers: &HashSet<Point>)->Option<usize>{
    let end = Point::new(width-1, height-1);

    let mut to_visit: BinaryHeap<MinHeapContainer<usize, Point>> = BinaryHeap::new();
    let mut visited: HashSet<Point> = HashSet::new();

    to_visit.push(MinHeapContainer{weight: 0, value: Point::new(0, 0)});

    loop {
        let Some(next) = to_visit.pop() else {return None};
        //println!("{:?}", next.value);
        if next.value == end {return Some(next.weight);}
        if !visited.insert(next.value.clone()){continue;}

        for d in vec![Direction::Up, Direction::Right, Direction::Down, Direction::Left]{
            if let Some(p) = next.value.step_direction(d, width, height){
                if visited.contains(&p){continue;}
                if barriers.contains(&p){continue;}
                to_visit.push(MinHeapContainer{weight: next.weight+1, value: p});
            }
        }
    }
}