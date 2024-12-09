use crate::utils::utils::get_input_file;

pub fn day9_main(second_part: bool, _extra_args: &Vec<String>){
    let input_file_str = get_input_file("d9.txt");
    if !second_part{
        part1(&input_file_str);
    }else{
        part2(&input_file_str);
    }
}

fn part1(input_file_str: &str){
    let mut front = FrontPtr::new(&input_file_str);
    let mut back = BackPtr::new(&input_file_str);

    let mut count:usize = 0;
    let mut checksum:usize = 0;

    'outer: while front.less_than_back(&back){
        count += 1;
        let front_v = front.next().expect("should stop in the middle-ish");
        if let Some(v) = front_v {
            checksum += (count-1)*v
        }else{
            loop {
                let back_v = back.next().expect("should stop in the middle-ish");
                if !front.less_than_back(&back){break 'outer}
                if let Some(v) = back_v {
                    checksum += (count-1)*v;
                    break;
                }
            }
        }
    }
    println!("");
    println!("Answer to part 1: {}", checksum);
}

struct FrontPtr<'a>{
    position: usize,
    remaining: usize,
    s: &'a str
}

impl<'a> FrontPtr<'a>{
    pub fn new(s: &'a str)->Self{
        Self {position: 0, s, remaining: (&s[0..1]).parse().expect("is number")}
    }
    
    pub fn next(&mut self)->Option<Option<usize>>{
        if self.position >= self.s.len() || self.remaining == 0{return None;}

        let is_val = self.position%2 == 0;
        let val = if is_val {Some(self.position/2)}else{None};
        
        self.remaining -= 1;
        while self.position<(self.s.len()-1) && self.remaining == 0{
            self.position+=1;
            self.remaining = (&self.s[self.position..self.position+1]).parse().expect("is number");
        }

        Some(val)
    }

    pub fn less_than_back(&self, other: &BackPtr)->bool{
        if self.position<other.position{
            true
        }else{
            let val:usize = (&self.s[self.position..self.position+1]).parse().expect("is number");
            (val-self.remaining)<other.remaining
        }
    }
}

struct BackPtr<'a>{
    position: usize,
    remaining: usize,
    s: &'a str
}

impl<'a> BackPtr<'a>{
    pub fn new(s: &'a str)->Self{
        let position = s.len()-1;
        let current_char = &s[position..position+1];
        let remaining = current_char.parse().expect("is number");
        Self{position, remaining, s}
    }

    pub fn next(&mut self)->Option<Option<usize>>{
        if self.position == 0 && self.remaining == 0{return None}

        let is_val = self.position%2 == 0;
        let val = if is_val {Some(self.position/2)}else{None};

        self.remaining -= 1;
        while self.position>0 && self.remaining == 0{
            self.position-=1;
            self.remaining = (&self.s[self.position..self.position+1]).parse().expect("is number");
        }
        Some(val)
    }

}


fn part2(input_file_str: &str){
    let blocks_raw:Vec<(usize, Option<usize>)> = input_file_str.chars().enumerate().map(|(i, c)|{
        let count:usize = (c as u8 - '0' as u8).into();
        let is_val = i%2 == 0;
        let val = if is_val {Some(i/2)}else{None};
        (count, val)
    }).collect();

    let mut i:usize = 0;
    let mut blocks: Vec<(usize, usize, Option<usize>)> = Vec::new();
    for (count, val) in blocks_raw.into_iter(){
        blocks.push((i, count, val));
        i+=count;
    }


    let mut space: Vec<(usize /* start */, usize /* count */)> = blocks.iter().filter_map(|(start, count, val)|if let Some(_)=val{None}else{Some((start.clone(), count.clone()))}).collect();
    let todo_blocks: Vec<(usize /* start */, usize /* count */, usize /* val */)> = blocks.iter().rev().filter_map(|(start, count, val)|if let Some(v)=val{Some((start.clone(), count.clone(), v.clone()))}else{None}).collect();
    let mut finalized_blocks: Vec<(usize /* start */, usize /* count */, usize /* val */)> = Vec::new();


    for (b_start, b_size, b_val) in todo_blocks{
        let mut best_space: Option<usize> = None;
        for i in 0..space.len(){
            let (space_start, space_size) = space[i];
            if space_start > b_start {continue;}
            if space_size < b_size {continue;}
            if let Some(idx) = best_space{
                let (best_start, _) = space[idx];
                if space_start < best_start {
                    best_space = Some(i);
                }
            }else{
                best_space = Some(i);
            }
        }

        if let Some(idx) = best_space {
            let (best_start, best_size) = space[idx];
            finalized_blocks.push((best_start, b_size, b_val));
            space[idx] = (best_start+b_size, best_size-b_size)
        }else{
            finalized_blocks.push((b_start, b_size, b_val));
        };
    }

    let mut checksum:usize = 0;
    for (start, size, val) in finalized_blocks.iter(){
        for i in *start..(start+size){
            checksum += i * val;
        }
    }

    println!("Answer to part 2: {}", checksum);
}