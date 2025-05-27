use core::f64;
use std::fs::File;
use std::io::{BufRead, BufReader}; 

// função objetivo é montada conforme a entrada

// fitness(s) = objetivo(s) - pressão * penalidade(s)
// penalidade(s) = penalidade_peso * (peso(s) - W) + 
//                 penalidade_incompat * numero_incompats

#[derive(Clone, Debug, Default)]
pub struct Objetivo{
    pub restricoes: Vec<Vec<bool>>,
    pub peso_max: i32,
    pub ingredientes: Vec<Ingrediente>,
}
#[derive(Clone, Debug, Default)]
pub struct Ingrediente{
    pub sabor: i32,
    pub peso: i32,
}

impl Objetivo {
    pub fn load_data(mut self, file_path: &'static str ){
        // let file_path  = "instances/ep01.dat";

        let file = File::open(file_path)
            .expect("Arquivo não pode ser aberto");

        let reader = BufReader::new(file);
        
        let mut num_ingredientes: i32;
        let mut num_incompativeis:i32;
        // reader.lines().next();
        for (line_num, line) 
            in reader.lines().enumerate() {
            
            match line {
                Ok(linha) => {
                    if line_num == 0{
                        let parse_num: Vec<i32> = linha
                            .split_whitespace()
                            .filter_map(|s| s.parse().ok())
                            .collect();
                        num_ingredientes = parse_num[0];
                        num_incompativeis = parse_num[1];
                        self.peso_max = parse_num[2];
                        self.ingredientes =  Objetivo::init_ingredientes(num_ingredientes);
                    }
                    println!("Linha {}: {}", line_num + 1, linha);

                }
                Err(erro) => {
                    eprintln!("Ocorreu um erro ao ler as linhas {:?}", erro);            
                }
            }
        }
    }
    fn init_ingredientes(num_ingredientes: i32) -> Vec<Ingrediente> {
        let mut ingredientes  = Vec::new();
        
        ingredientes.resize(num_ingredientes as usize, 
                    Ingrediente::default());
        ingredientes
    }
}