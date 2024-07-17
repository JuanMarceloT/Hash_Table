
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::time::{Duration, Instant};
use std::io::{BufWriter, Write};

#[derive(Debug, Clone)]
struct Player {
    id: i32,
    name: String,
    player_positions: Vec<String>,
}

fn insert(num: Player, modulo: usize, hash_table: &mut[Vec<Player>]){
    let index = (num.id as usize) % modulo;
    hash_table[index].push(num);
}

fn search(id: i32, modulo: usize, hash_table: &[Vec<Player>], mut consultas : Option< &mut i32>) ->  Option<Player> {
    let index = (id as usize) % modulo;
    
    for player in &hash_table[index] {

        match consultas {
            Some(ref mut n) => {
                **n += 1;
            },
            None => {},
        }

        if player.id == id {
            return Some(player.clone());
        }
    }
    None
}


fn read_csv<P, F>(filename: P, mut func: F) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
    F: FnMut(Player),
{
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
            
            func(player);
        }
    }
    Ok(())
}

fn read_csv_id<P, F>(filename: P, mut func: F) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
    F: FnMut(i32),
{
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        let id = record[0].parse::<i32>()?;
        func(id);
    }
    Ok(())
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

fn stats(modulo: usize) -> std::io::Result<()> {

    let tempo_construcao_tabela: Duration = measure_time(|| {
        const ARRAY_REPEAT_VALUE: Vec<Player> = Vec::new();
        let hash_table: Vec<Vec<Player>> = vec![Vec::new(); modulo];
        let ocupation: i32 = 0;
    });
    const ARRAY_REPEAT_VALUE: Vec<Player> = Vec::new();
    let mut hash_table: Vec<Vec<Player>> = vec![Vec::new(); modulo];
    let mut ocupation: i32 = 0;
    

    let _result = read_csv("players.csv", |player| {
        insert(player, modulo, &mut hash_table);
        ocupation += 1;
    });


    let taxa_ocupacacao : f32 = ocupation as f32 / modulo as f32;

    let mut maior_list : i32 = 0;
    let mut media_list : f32 = 0.0;
    let mut media_list_ocupacacao : i32 = 0;

    for i in 0..modulo {
        if hash_table[i].len() > maior_list as usize{
            maior_list = hash_table[i].len() as i32;
        }
        if hash_table[i].len() > 0 {
            media_list += hash_table[i].len() as f32;
            media_list_ocupacacao += 1;
        }else {
            //println!("tem vazio");
        }
    }

    media_list = media_list / media_list_ocupacacao as f32;


    let file = File::create(format!("experimento{}.txt", modulo))?;

    let mut writer = BufWriter::new(file);

    {let output = format!("PARTE1: ESTATISTICAS DA TABELA HASH
    TEMPO DE CONSTRUCAO DA TABELA: {:?}
    TAXA DE OCUPACAO: {:?}
    TAMANHO MAXIMO DE LISTA: {:?}
    TAMANHO MEDIO DE LISTA: {:?}\n", tempo_construcao_tabela, taxa_ocupacacao, maior_list, media_list);
    writer.write_all(output.as_bytes());
    }

    let mut max_consultas: i32 = 0;
    let mut media_consultas:f32 = 0.0;
    let mut media_consultas_ocupacao: i32 = 0;

    
    let tempo_todas_consultas : Duration = measure_time(|| {
        let mut consultas_stats: i32 = 0;
        let _result = read_csv_id("consultas.csv", |id| {
            match search(id, modulo, &hash_table, Some(&mut consultas_stats)) {
                Some(player) => {

                    if consultas_stats > max_consultas {
                        max_consultas = consultas_stats;
                    }
        
                    media_consultas += consultas_stats as f32;
                    media_consultas_ocupacao += 1;
                    //println!("{:?} {:?} {:?}", id, player.name, consultas_stats);

                    consultas_stats = 0;

                }, 

                None => {}//println!("{:?} NÃO ENCONTRADO {:?}", id, consultas)
            }

            
            
        });
    });
    
    {let output = format!("\nPARTE 2: ESTATISTICAS DAS CONSULTAS
    TEMPO PARA REALIZACAO DE TODAS CONSULTAS: {:?}\n", tempo_todas_consultas);
    writer.write_all(output.as_bytes());
    }


    let _result = read_csv_id("consultas.csv", |id| {
        let mut consultas_stats: i32 = 0;
        match search(id, modulo, &hash_table, Some(&mut consultas_stats)) {
            Some(player) => {

                if consultas_stats > max_consultas {
                    max_consultas = consultas_stats;
                }
    
                media_consultas += consultas_stats as f32;
                media_consultas_ocupacao += 1;
                //println!("{:?} {:?} {:?}", id, player.name, consultas_stats);
                
                {let output = format!("{:?} {:?} {:?}\n", id, player.name, consultas_stats);
                writer.write_all(output.as_bytes());
                }


                consultas_stats = 0;

            }, 

            None => {let output = format!("{:?} NAO ENCONTRADO {:?}\n", id, consultas_stats);
            writer.write_all(output.as_bytes());
            }
        }

        
        
    });

    media_consultas = media_consultas / media_consultas_ocupacao as f32;

    {let output = format!("    MAXIMO NUMERO DE TESTES POR NOME ENCONTRADO: {:?}
    MEDIA NUMERO DE TESTES POR NOME ENCONTRADO: {:?}", max_consultas, media_consultas);
            writer.write_all(output.as_bytes());
    }

    writer.flush()?;

    println!("Write Operation Successful");
    Ok(())
}

fn main() {

    let _ = stats(997);
    let _ = stats(1999);
    let _ = stats(3989);
    let _ = stats(7993);
}
