use super::Problema;
use rand::Rng;

const PEN_PESO: f64 = 100.0;
const PEN_INCOMPATIVEIS: f64 = 100.0;
const AR: f64 = 0.5;

#[derive(Clone, Debug, Default)]
pub struct Solucao {
    // vetor de inteiros, onde cada inteiro é o
    // índice de um ingrediente que está presente naquela solução
    pub ingredientes: Vec<usize>,

    // valor fitness da mistura
    pub resultado: f64,
}

pub fn calcula_fitness(
    ingredientes: &Vec<usize>, 
    problema: &Problema,
    // pressao = 1 => sem pressão 
    pressao: f64) -> f64 {

    let mut sabores: f64 = 0.0;
    let mut peso_acomulado: i32 = 0;
    let mut incompativeis: f64 = 0.0;
    
    // pega todos os valores
    for ing in ingredientes.iter().copied(){
        sabores += problema.ingredientes[ing].sabor as f64;
        peso_acomulado += problema.ingredientes[ing].peso;

        for inc in ingredientes.iter().copied(){
            if problema.restricoes[ing as usize].contains(&inc){
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

pub fn populacao_inicial(tamanho_populacao: usize, pressao_inicial: f64, problema: &Problema) -> Vec<Solucao> {
    let mut populacao: Vec<Solucao> = Vec::new();

    for _j in 0..tamanho_populacao{
        let mut ingredientes:Vec<usize> = Vec::new();
        for i in 0..problema.num_ingred{
            let r = rand::thread_rng().gen_range(0.0, 1.0);
            if r >= AR{
                ingredientes.push(i);
            }
        }
        populacao.push(Solucao::new(ingredientes, problema, pressao_inicial));
    }
    populacao
}

impl Solucao{
    pub fn new(ingredientes: Vec<usize>, problema: &Problema, pressao: f64) -> Self {
        let resultado = calcula_fitness(&ingredientes, problema, pressao);
        Solucao { 
            ingredientes, 
            resultado,
        }
    }
}
