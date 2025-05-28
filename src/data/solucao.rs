use super::Problema;

const PEN_PESO: f64 = 100.0;
const PEN_INCOMPATIVEIS: f64 = 100.0;

#[derive(Clone, Debug)]
pub struct Solucao {
    // vetor de inteiros, onde cada inteiro é o
    // índice de um ingrediente que está presente naquela solução
    pub ingredientes: Vec<i32>,

    // valor fitness da mistura
    pub resultado: f64,
}

pub fn calcula_fitness(
    ingredientes: &Vec<i32>, 
    problema: &Problema,
    // pressao = 1 => sem pressão 
    pressao: f64) -> f64 {

    let mut sabores: f64 = 0.0;
    let mut peso_acomulado: i32 = 0;
    let mut incompativeis: f64 = 0.0;
    
    // pega todos os valores
    for ing in ingredientes.iter().copied(){
        sabores += problema.ingredientes[ing as usize].sabor as f64;
        peso_acomulado += problema.ingredientes[ing as usize].peso;

        for inc in ingredientes.iter().copied(){
            if problema.restricoes[ing as usize].contains(&(inc as usize)){
                println!("ingrediente {}, incompativel {}", ing, inc);
                incompativeis += 1.0;
            }
        }
    }

    // realiza os calculos para calcular o fitness
    let diff = peso_acomulado - problema.peso_max;
    let penalidade = PEN_PESO *  if diff >=0 {diff as f64} else {0.0}
                    + PEN_INCOMPATIVEIS * incompativeis;
    
    let fitness = sabores - pressao * penalidade;
    fitness
}

impl Solucao{
    pub fn new(ingredientes: Vec<i32>, problema: &Problema, pressao: f64) -> Self {
        let resultado = calcula_fitness(&ingredientes, problema, pressao);
        Solucao { 
            ingredientes, 
            resultado,
        }
    }
}
