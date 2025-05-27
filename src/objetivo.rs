use core::f64;
use std::fs::File;
use std::io::{self, BufRead, BufReader}; 

// função objetivo é montada conforme a entrada

// fitness(s) = objetivo(s) - pressão * penalidade(s)
// penalidade(s) = penalidade_peso * (peso(s) - W) + 
//                 penalidade_incompat * numero_incompats

#[derive(Clone, Debug)]
pub struct Objetivo{
    pub restricoes: Vec<Vec<bool>>,
    pub peso_max: i32,
    pub ingredientes: Vec<Ingrediente>,
}
#[derive(Clone, Debug)]
pub struct Ingrediente{
    pub sabor: i32,
    pub peso: i32,
}

impl Objetivo {
    pub fn load_data(){
        let file_path = "instances/ep01.dat";

        let file = File::open(file_path);

        match file {
            Ok(file_handle) => {
                let reader = BufReader::new(file_handle);

                for (line_num, line) in reader.lines().enumerate() {
                    println!("Linha {}: {:?}", line_num + 1, line);
                }
                
            },
            Err(erro) => {
                eprintln!("Ocorreu um erro ao abrir o arquivo {:?}", erro);
                eprintln!("arquivo: {}", file_path);
            }
            
        }
    }
}