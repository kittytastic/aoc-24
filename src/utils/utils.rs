use std::path::Path;
use std::{fs, usize};

// Read an input file from the input data directory
pub fn get_input_file(file_name: &str) -> String{
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("input_data").join(file_name);
    if !path.is_file(){
        println!("ERROR: file doesn't exist {:?}", path);
        std::process::exit(1);
    }

    return fs::read_to_string(path).expect("Should be able to read file");
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    pub fn turn_90_clockwise(&self)->Direction{
        match self{
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::UpRight => Direction::DownRight,
            Direction::DownRight => Direction::DownLeft,
            Direction::DownLeft => Direction::UpLeft,
            Direction::UpLeft => Direction::UpRight,
        }
    }
    
    pub fn turn_90_anticlockwise(&self)->Direction{
        match self{
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::UpRight => Direction::UpLeft,
            Direction::UpLeft => Direction::DownLeft,
            Direction::DownLeft => Direction::DownRight,
            Direction::DownRight => Direction::UpRight,
        }
    }
    
    pub fn _turn_180(&self)->Direction{
        match self{
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::UpRight => Direction::DownLeft,
            Direction::DownLeft => Direction::UpRight,
            Direction::DownRight => Direction::UpLeft,
            Direction::UpLeft => Direction::DownRight,
        }
    }
    
    pub fn turn_45_clockwise(&self)->Direction{
        match self{
            Direction::Up => Direction::UpRight,
            Direction::UpRight => Direction::Right,
            Direction::Right => Direction::DownRight,
            Direction::DownRight => Direction::Down,
            Direction::Down => Direction::DownLeft,
            Direction::DownLeft => Direction::Left,
            Direction::Left => Direction::UpLeft,
            Direction::UpLeft => Direction::Up,
        }
    }
    
    pub fn turn_45_anticlockwise(&self)->Direction{
        match self{
            Direction::Up => Direction::UpLeft,
            Direction::UpLeft => Direction::Left,
            Direction::Left => Direction::DownLeft,
            Direction::DownLeft => Direction::Down,
            Direction::Down => Direction::DownRight,
            Direction::DownRight => Direction::Right,
            Direction::Right => Direction::UpRight,
            Direction::UpRight => Direction::Up,
        }
    }
}

pub struct Grid<T> {
    data: Vec<Vec<T>>,
    height: usize,
    width: usize
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone)]
pub struct GridPoint<'a, T>{
    grid: &'a Grid<T>,
    point: Point
}

impl Grid<char>{
    pub fn new_from_str(txt: &str) -> Grid<char>{
        let input_data: Vec<Vec<char>> = txt.lines().map(|line: &str|{
            line.chars().map(|c|c).collect::<Vec<char>>()
        }).collect::<Vec<_>>();

        Grid::new(input_data)
    }
}

impl<T> Grid<T>{
    pub fn new(data: Vec<Vec<T>>) -> Self{
        let width = data[0].len();
        let height = data.len();
        Self {data, height, width}
    }
    
    pub fn height(&self)->usize{
        self.height
    }

    pub fn width(&self)->usize{
        self.width
    }

    pub fn get(&self, x:usize, y:usize)->&T{
        &self.data[y][x] 
    }

    pub fn get_mut(&mut self, x:usize, y:usize)->&mut T{
        &mut self.data[y][x] 
    }

    pub fn set(&mut self, x:usize, y: usize, value: T) {
        self.data[y][x] = value
    }

    pub fn get_point<'a>(&'a self, x:usize, y:usize) -> Option<GridPoint<'a, T>> {
        if x>=self.width {return None;}
        if y>=self.height {return None;}
        Some(GridPoint {grid: &self, point: Point{x, y}})
    }

    pub fn set_val_at_foreign_point<'a, S>(&mut self, value: T, point: &GridPoint<'a, S>)->Result<(), ()>{
        if point.get_x() > self.width{return Err(())}
        if point.get_y() > self.height{return Err(())}
        *self.get_mut(point.get_x(), point.get_y()) = value;
        Ok(())
    }
    
    pub fn get_val_at_foreign_point<'a, S>(&self, point: &GridPoint<'a, S>)->Result<&T, ()>{
        if point.get_x() > self.width{return Err(())}
        if point.get_y() > self.height{return Err(())}
        Ok(&self.get(point.get_x(), point.get_y()))
    }

    pub fn at_point(&self, point: &Point)->&T{
        self.get(point.get_x(), point.get_y())
    }
    
}

impl<T: Clone> Grid<T>{
    pub fn new_from(value: T, height: usize, width: usize)->Self{
        let data: Vec<Vec<T>> = (0..height).map(|_|{(0..width).map(|_|value.clone()).collect()}).collect();
        Self{data, height, width}
    }
}

impl<T: Eq> Grid<T>{
    pub fn find_first_occurrence(&self, look_for: &T)->Option<GridPoint<T>>{
        for y in 0..self.height(){
            for x in 0..self.width(){
                if *self.get(x, y) == *look_for{
                    return Some(GridPoint{grid: &self, point: Point{x, y}})
                }
            }
        }
        None
    }
    
    pub fn count_all_occurrence_of(&self, look_for: &T)->usize{
        let mut c = 0;
        for y in 0..self.height(){
            for x in 0..self.width(){
                if *self.get(x, y) == *look_for{
                    c+=1;
                }
            }
        }
        c
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Grid<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height{
            for x in 0..self.width{
                write!(f, "{}", self.get(x, y))?
            }
            writeln!(f, "")?
        }
        writeln!(f, "height: {}  width: {}", self.height, self.width)
    }
}

impl Point{
    pub fn new(x:usize, y:usize)->Self{
        Point { x, y }
    }

    pub fn get_x(&self)->usize{
        self.x
    }

    pub fn get_y(&self)->usize{
        self.y
    }

    pub fn left(&self, width: usize, height: usize) -> Option<Point>{
        self.left_by(1 ,width, height)
    }

    pub fn right(&self, width: usize, height: usize) -> Option<Point> {
        self.right_by(1 ,width, height)
    }

    pub fn up(&self, width: usize, height: usize) -> Option<Point> {
        self.up_by(1 ,width, height)
    }

    pub fn down(&self, width: usize, height: usize) -> Option<Point> {
        self.down_by(1 ,width, height)
    }
    
    pub fn up_left(&self, width: usize, height: usize) -> Option<Point>{
        self.up_left_by(1 ,width, height)
    }

    pub fn up_right(&self, width: usize, height: usize) -> Option<Point> {
        self.up_right_by(1 ,width, height)
    }

    pub fn down_left(&self, width: usize, height: usize) -> Option<Point> {
        self.down_left_by(1 ,width, height)
    }

    pub fn down_right(&self, width: usize, height: usize) -> Option<Point> {
        self.down_right_by(1 ,width, height)
    }
    
    pub fn left_by(&self, step: usize, _width: usize, _height: usize) -> Option<Point>{
        if self.x < step {return None}
        Some(Point{x: self.x-step, y:self.y})
    }

    pub fn right_by(&self, step: usize, width: usize, _height: usize) -> Option<Point> {
        if self.x + step >= width{return None}
        Some(Self{x: self.x+step, y: self.y})
    }

    pub fn up_by(&self, step: usize, _width: usize, _height: usize) -> Option<Point> {
        if self.y < step {return None}
        Some(Self{x: self.x, y: self.y-step})
    }

    pub fn down_by(&self, step: usize, _width: usize, height: usize) -> Option<Point> {
        if self.y + step >= height{return None}
        Some(Self{x: self.x, y: self.y+step})
    }
    
    pub fn up_left_by(&self, step: usize, _width: usize, _height: usize) -> Option<Point>{
        if self.x < step {return None}
        if self.y < step {return None}
        Some(Self{x: self.x-step, y: self.y-step})
    }

    pub fn up_right_by(&self, step: usize, width: usize, _height: usize) -> Option<Point> {
        if self.x + step >= width{return None}
        if self.y < step {return None}
        Some(Self{x: self.x+step, y: self.y-step})
    }

    pub fn down_left_by(&self, step: usize, _width: usize, height: usize) -> Option<Point> {
        if self.y + step >= height{return None}
        if self.x < step {return None}
        Some(Self{x: self.x-step, y: self.y+step})
    }

    pub fn down_right_by(&self, step: usize, width: usize, height: usize) -> Option<Point> {
        if self.y + step >= height {return None}
        if self.x + step  >= width {return None}
        Some(Self{x: self.x+step, y: self.y+step})
    }

    pub fn step_direction(&self, dir: Direction, width: usize, height: usize)->Option<Point>{
        match dir{
            Direction::Up => self.up(width, height),
            Direction::Right => self.right(width, height),
            Direction::Down => self.down(width, height),
            Direction::Left => self.left(width, height),
            Direction::UpRight => self.up_right(width, height),
            Direction::DownRight => self.down_right(width, height),
            Direction::DownLeft => self.down_left(width, height),
            Direction::UpLeft => self.up_left(width, height),
        }
    }
    
    pub fn _step_direction_by(&self, dir: Direction, step: usize, width: usize, height: usize)->Option<Point>{
        match dir{
            Direction::Up => self.up_by(step, width, height),
            Direction::Right => self.right_by(step, width, height),
            Direction::Down => self.down_by(step, width, height),
            Direction::Left => self.left_by(step, width, height),
            Direction::UpRight => self.up_right_by(step, width, height),
            Direction::DownRight => self.down_right_by(step, width, height),
            Direction::DownLeft => self.down_left_by(step, width, height),
            Direction::UpLeft => self.up_left_by(step, width, height),
        }
    }

    pub fn move_by(&self, d_x: i64, d_y: i64, width: usize, height: usize)->Option<Point>{
        let self_x_i64:i64 = self.x.try_into().expect("Nice numbers");
        let self_y_i64:i64 = self.y.try_into().expect("Nice numbers");
        
        let new_x: i64 = self_x_i64 + d_x;
        let new_y: i64 = self_y_i64 + d_y;

        if new_x<0 || new_x>=width.try_into().expect("Nice numbers"){return None}
        if new_y<0 || new_y>=height.try_into().expect("Nice numbers"){return None}

        Some(Point{x: new_x.try_into().expect("Nice numbers"), y:new_y.try_into().expect("Nice numbers")})
    }
     
    pub fn move_by_modulo(&self, d_x: i64, d_y: i64, width: usize, height: usize)->Point{
        let self_x_i64:i64 = self.x.try_into().expect("Nice numbers");
        let self_y_i64:i64 = self.y.try_into().expect("Nice numbers");
        let width_i64:i64 = width.try_into().expect("Nice numbers");
        let height_i64:i64 = height.try_into().expect("Nice numbers");

        let new_x: i64 = (self_x_i64 + d_x)%width_i64;
        let new_y: i64 = (self_y_i64 + d_y)%height_i64;
        let new_x = if new_x<0{width_i64+new_x}else{new_x};
        let new_y = if new_y<0{height_i64+new_y}else{new_y};

        Point{x: new_x.try_into().expect("Nice numbers"), y:new_y.try_into().expect("Nice numbers")}
    }

    pub fn to_i64(&self)->(i64, i64){
        (self.x.try_into().expect("Nice numbers"), self.y.try_into().expect("Nice numbers"))
    }

    pub fn delta(&self, other: &Self)->(i64, i64){
        let (self_x, self_y) = self.to_i64();
        let (other_x, other_y) = other.to_i64();
        (self_x-other_x, self_y-other_y)
    }
}

impl<'a, T> GridPoint<'a, T>{
    pub fn value(&'a self) -> &'a T{
        &self.grid.get(self.point.get_x(), self.point.get_y())
    }

    pub fn get_x(&self)->usize{
       self.point.get_x() 
    }
    
    pub fn get_y(&self)->usize{
       self.point.get_y()
    }

    pub fn left(&self) -> Option<GridPoint<'a, T>>{
        self.point.left(self.grid.width(), self.grid.height()).map(|p|GridPoint { grid: &self.grid, point: p })        
    }

    pub fn right(&self) -> Option<GridPoint<'a, T>> {
        self.point.right(self.grid.width(), self.grid.height()).map(|p|GridPoint { grid: &self.grid, point: p })        
    }

    pub fn up(&self) -> Option<GridPoint<'a, T>> {
        self.point.up(self.grid.width(), self.grid.height()).map(|p|GridPoint { grid: &self.grid, point: p })        
    }

    pub fn down(&self) -> Option<GridPoint<'a, T>> {
        self.point.down(self.grid.width(), self.grid.height()).map(|p|GridPoint { grid: &self.grid, point: p })        
    }
    
    pub fn up_left(&self) -> Option<GridPoint<'a, T>>{
        self.point.up_left(self.grid.width(), self.grid.height()).map(|p|GridPoint { grid: &self.grid, point: p })        
    }

    pub fn up_right(&self) -> Option<GridPoint<'a, T>> {
        self.point.up_right(self.grid.width(), self.grid.height()).map(|p|GridPoint { grid: &self.grid, point: p })        
    }

    pub fn down_left(&self) -> Option<GridPoint<'a, T>> {
        self.point.down_left(self.grid.width(), self.grid.height()).map(|p|GridPoint { grid: &self.grid, point: p })        
    }

    pub fn down_right(&self) -> Option<GridPoint<'a, T>> {
        self.point.down_right(self.grid.width(), self.grid.height()).map(|p|GridPoint { grid: &self.grid, point: p })        
    }

    pub fn step_direction(&self, dir: Direction)->Option<GridPoint<'a, T>>{
        match dir{
            Direction::Up => self.up(),
            Direction::Right => self.right(),
            Direction::Down => self.down(),
            Direction::Left => self.left(),
            Direction::UpRight => self.up_right(),
            Direction::DownRight => self.down_right(),
            Direction::DownLeft => self.down_left(),
            Direction::UpLeft => self.up_left(),
        }
    }
}

impl<'a, T:Clone> GridPoint<'a, T>{
    pub fn right_slice(&self, slice_len: usize) -> Option<Vec<T>> {
        if self.point.get_x() > self.grid.width()-slice_len{return None}
        Some((0..slice_len).into_iter().map(|d|{self.grid.get(self.point.get_x()+d, self.point.get_y()).clone()}).collect::<Vec<T>>())
    }
    
    pub fn down_slice(&self, slice_len: usize) -> Option<Vec<T>> {
        if self.get_y() > self.grid.height()-slice_len{return None}
        Some((0..slice_len).into_iter().map(|d|{self.grid.get(self.get_x(), self.get_y()+d).clone()}).collect::<Vec<T>>())
    }
    
    pub fn down_right_slice(&self, slice_len: usize) -> Option<Vec<T>> {
        if self.get_y() > self.grid.height()-slice_len{return None}
        if self.get_x() > self.grid.width()-slice_len{return None}
        Some((0..slice_len).into_iter().map(|d|{self.grid.get(self.get_x()+d, self.get_y()+d).clone()}).collect::<Vec<T>>())
    }
    
    pub fn down_left_slice(&self, slice_len: usize) -> Option<Vec<T>> {
        if self.get_y() > self.grid.height()-slice_len{return None}
        if self.get_x() < slice_len-1{return None}
        Some((0..slice_len).into_iter().map(|d|{self.grid.get(self.get_x()-d, self.get_y()+d).clone()}).collect::<Vec<T>>())
    }
} 

impl<'a, T: std::fmt::Display> std::fmt::Display for GridPoint<'a, T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.get_x(), self.get_y())
    }
}

impl std::fmt::Display for Point{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.get_x(), self.get_y())
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_grid_slice_inbound() {
        //assert_eq!(bad_add(1, 2), 3);
    }
}