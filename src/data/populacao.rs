use rand::Rng;

use super::{Problema, Solucao};

pub struct Populacao {
    pub populacao: Vec<Solucao>,
    pub ar: f64,
}
