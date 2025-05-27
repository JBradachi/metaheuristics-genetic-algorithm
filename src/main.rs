use genetic_algorithm::{
    genetico, objetivo::{Objetivo}, solucao
};

fn main() {
    println!("Hello, world!");
    let mut o1: Objetivo = Objetivo::default();
    println!("objetivo default {:?}", o1);
    let mut vetor: Vec<bool> = Vec::new();
    vetor.push(true);
    vetor.push(true);
    vetor.push(false);
    o1.restricoes.push(vetor);
    println!("objetivo default {:?}", o1);

}
