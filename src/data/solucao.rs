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

pub fn calcula_fitness(ingredientes: &Vec<usize>, p: &Problema) -> f64 {
    let mut sabores: f64 = 0.0;
    let mut peso_acumulado: i32 = 0;
    let mut incompativeis: f64 = 0.0;

    for ing in ingredientes.iter().copied() {
        sabores += p.ingredientes[ing].sabor as f64;
        peso_acumulado += p.ingredientes[ing].peso;

        for inc in ingredientes.iter().copied() {
            if p.restricoes[ing as usize].contains(&inc) {
                incompativeis += 1.0;
            }
        }
    }
    let diff = peso_acumulado - p.peso_max;
    let diff = if diff >= 0 { diff as f64 } else { 0.0 };
    let penalidade = PEN_PESO * diff + PEN_INCOMPATIVEIS * incompativeis;
    sabores - p.pressao * penalidade
}

pub fn populacao_inicial(tam_populacao: usize, p: &Problema) -> Vec<Solucao> {
    let mut populacao: Vec<Solucao> = Vec::new();

    for _j in 0..tam_populacao {
        let mut ingredientes: Vec<usize> = Vec::new();
        for i in 0..p.num_ingred {
            let r = rand::thread_rng().gen_range(0.0, 1.0);
            if r >= AR {
                ingredientes.push(i);
            }
        }
        populacao.push(Solucao::new(ingredientes, p));
    }
    populacao
}

impl Solucao {
    pub fn new(ingredientes: Vec<usize>, problema: &Problema) -> Self {
        let resultado = calcula_fitness(&ingredientes, problema);
        Solucao {
            ingredientes,
            resultado,
        }
    }
    pub fn update(&mut self, problema: &Problema) {
        self.resultado = calcula_fitness(&self.ingredientes, problema);
    }
}
