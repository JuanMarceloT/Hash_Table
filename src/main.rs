

fn insert(num: i32, module: usize, hash_table: &mut[Vec<i32>]){
    let index = (num as usize) % module;
    hash_table[index].push(num);
}

fn search(num: i32, module: usize, hash_table: &[Vec<i32>]) ->  Option<i32> {
    let index = (num as usize) % module;
    for &val in &hash_table[index] {
        if val == num {
            return Some(val);
        }
    }
    None
}


fn main() {
    println!("Hello, world!");

    const MODULE : usize = 25; 
    const ARRAY_REPEAT_VALUE: Vec<i32> = Vec::new();
    let mut hash_table : [Vec<i32>; MODULE] = [ARRAY_REPEAT_VALUE; MODULE];

    insert(12, MODULE, &mut hash_table);
    insert(2, MODULE, &mut hash_table);
    insert(90, MODULE, &mut hash_table);
    insert(15, MODULE, &mut hash_table);
    insert(28, MODULE, &mut hash_table);

    println!("{:?}", search(13, MODULE, &hash_table));
    println!("{:?}", hash_table);
}
