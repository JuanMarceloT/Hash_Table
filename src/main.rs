use std::time::{Duration, Instant};

fn insert(num: i32, module: usize, hash_table: &mut[Vec<i32>]){
    let index = (num as usize) % module;
    hash_table[index].push(num);
}

fn search(id: i32, module: usize, hash_table: &[Vec<i32>]) ->  Option<i32> {
    let index = (id as usize) % module;
    
    for nums in &hash_table[index] {
        return Some(*nums);
    }
    None
}

fn measure_time<F>(func: F) -> Duration
where
    F: FnOnce(),
{
    let start = Instant::now();
    func();
    let duration = start.elapsed();
    //println!("Tempo gasto: {:?}", duration);
    duration
}
fn main() {

    const MODULE : usize = 25; 
    const ARRAY_REPEAT_VALUE: Vec<i32> = Vec::new();
    let mut hash_table : [Vec<i32>; MODULE] = [ARRAY_REPEAT_VALUE; MODULE];
}
