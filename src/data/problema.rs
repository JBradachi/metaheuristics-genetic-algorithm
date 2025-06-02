use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, Default)]
pub struct Ingrediente {
    pub sabor: i32,
    pub peso: i32,
}

#[derive(Clone, Debug, Default)]
/// Estrutura que agrupa os dados de entrada que descrevem o problema
pub struct Problema {
    /// Peso máximo que todos os ingredientes juntos podem ter
    pub peso_max: i32,

    /// Grafo que descreve as incompatibilidades entre os ingredientes.
    /// Armazenado como uma lista de adjacência, essencialmente
    pub restricoes: Vec<HashSet<usize>>,

    /// Ingredientes disponíveis. Cada um é identificado por seu índice
    pub ingredientes: Vec<Ingrediente>,
    pub num_ingred: usize,
}

/// Lê um vetor de inteiros da string dada
fn read_integers(text: &str) -> Vec<i32> {
    text.split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect()
}

impl Problema {
    /// Carrega os dados do problema de um arquivo externo
    pub fn load_from(file_path: &str) -> Self {
        let file = File::open(file_path).expect("Arquivo não pode ser aberto");
        let mut lines = BufReader::new(file).lines();

        // Primeira linha:
        // N: número de ingredientes
        // I: número de ingredientes incompatíveis
        // W: peso máximo
        let first_line = lines.next().unwrap().unwrap(); // sabemos que essa linha existe
        let nums = read_integers(&first_line);
        let num_ingred = nums[0] as usize;
        let num_incompat = nums[1];
        let peso_max = nums[2];

        // Inicialização do vetor de ingredientes com o tamanho certo
        let mut ingredientes: Vec<Ingrediente> = Vec::with_capacity(num_ingred);
        ingredientes.resize_with(num_ingred, Default::default);

        // Inicialização do grafo de incompatibilidades com o tamanho certo
        let mut restricoes = Vec::with_capacity(num_ingred);
        restricoes.resize_with(num_ingred, HashSet::new);

        // Próximos N números: sabores
        lines.next(); // linha em branco
        let mut i = 0;
        while i < num_ingred {
            let line = lines.next().unwrap().unwrap();
            for &sabor in read_integers(&line).iter() {
                ingredientes[i].sabor = sabor;
                i += 1;
            }
        }

        // Próximos N números: pesos
        lines.next(); // linha em branco
        let mut i = 0;
        while i < num_ingred {
            let line = lines.next().unwrap().unwrap();
            for &peso in read_integers(&line).iter() {
                ingredientes[i].peso = peso;
                i += 1;
            }
        }

        // Próximas I linhas: pares j,k de incompatibilidades
        lines.next(); // linha em branco
        for _ in 0..num_incompat {
            let line = lines.next().unwrap().unwrap();
            let pair = read_integers(&line);
            let j = pair[0] as usize - 1;
            let k = pair[1] as usize - 1;
            restricoes[j].insert(k);
            // restricoes[k].insert(j); se precisar descomente, com isso gera um grafo sem direção
        }
        Problema {
            peso_max,
            restricoes,
            ingredientes,
            num_ingred,
        }
    }
}
