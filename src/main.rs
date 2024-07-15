
use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Clone)]
struct Player {
    id: i32,
    name: String,
    player_positions: Vec<String>,
}

fn insert(num: Player, module: usize, hash_table: &mut[Vec<Player>]){
    let index = (num.id as usize) % module;
    hash_table[index].push(num);
}

fn search(id: i32, module: usize, hash_table: &[Vec<Player>]) ->  Option<Player> {
    let index = (id as usize) % module;
    
    for player in &hash_table[index] {
        if player.id == id {
            return Some(player.clone());
        }
    }
    None
}


fn read_csv<P: AsRef<Path>>(filename: P) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        if record.len() == 3 {
            let player = Player {
                id: record[0].parse::<i32>()?,
                name: record[1].to_string(),
                player_positions: record[2].split(',').map(|s| s.to_string()).collect(),
            };
            println!("{:?}", player.id);
        } 
    }
    Ok(())
}


fn main() {

    const MODULE : usize = 25; 
    const ARRAY_REPEAT_VALUE: Vec<Player> = Vec::new();
    let mut hash_table : [Vec<Player>; MODULE] = [ARRAY_REPEAT_VALUE; MODULE];

    let messi : Player = Player{
        id: 13,
        name: "Lionel Messi".to_string(),
        player_positions: vec!["Forward".to_string(), "Midfielder".to_string()]
    };

    read_csv("players.csv");
    insert(messi, MODULE, &mut hash_table);

    println!("{:?}", search(13, MODULE, &hash_table).unwrap().name);

}
