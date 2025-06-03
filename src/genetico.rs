use rand::{Rng, distributions::range};

use crate::data::{Problema, Solucao, problema, solucao::populacao_inicial};

const AR: f64 = 0.5;

fn get_melhor_solucao(populacao: &Vec<Solucao>) -> Solucao {
    let mut melhor_solucao: &Solucao;
    let mut tmp = populacao.iter();
    melhor_solucao = tmp.next().unwrap();
    while let Some(i) = tmp.next() {
        if i.resultado > melhor_solucao.resultado {
            melhor_solucao = i;
        }
    }
    melhor_solucao.clone()
}

fn crossover(problema: &Problema, pais: Vec<Solucao>) -> Vec<Solucao> {
    let mut filhos = Vec::new();
    while filhos.len() < pais.len() {
        let mut rng = rand::thread_rng();
        let p0 = rng.choose(&pais).unwrap();
        let p1 = rng.choose(&pais).unwrap();
        let corte: usize = rng.gen_range(1, problema.num_ingred - 2);

        let mut f0 = Vec::new();
        let mut f1 = Vec::new();
        for i in 0..p0.ingredientes.len() {
            let ing0 = p0.ingredientes[i];
            let ing1 = p1.ingredientes[i];
            if ing0 < corte {
                f0.push(ing0);
            } else {
                f1.push(ing0);
            }
            if ing1 < corte {
                f1.push(ing1);
            } else {
                f0.push(ing1);
            }
        }
        let s0 = Solucao::new(f0, problema);
        let s1 = Solucao::new(f1, problema);
        filhos.push(s0);
        filhos.push(s1);
    }
    filhos
}

fn bit_flip(problema: &Problema, mut populacao: Vec<Solucao>) -> Vec<Solucao> {
    let mut rng = rand::thread_rng();
    let mut tmp = populacao.iter_mut();
    while let Some(i) = tmp.next() {
        let r = rng.gen_range(0.0, 1.0);

        //verifica se receberá a mutação
        if r >= AR {
            //faz a mutação
            let gene_mutado: usize = rng.gen_range(1, problema.num_ingred - 2);
            if i.ingredientes.contains(&gene_mutado) {
                i.ingredientes.retain(|&x| x != gene_mutado);
            } else {
                i.ingredientes.push(gene_mutado);
            }
            i.update(problema);
        }
    }
    populacao
}

fn mutacao(
    problema: &Problema,
    pais: Vec<Solucao>,
    filhos: Vec<Solucao>,
) -> (Vec<Solucao>, Vec<Solucao>) {
    let pais_mutados: Vec<Solucao> = bit_flip(problema, pais.clone());

    let filhos_mutados: Vec<Solucao> = bit_flip(problema, filhos.clone());

    (pais_mutados, filhos_mutados)
}

fn realiza_torneio(competidores: Vec<Solucao>) -> Solucao {
    let mut campeao_atual: Solucao = competidores[0].clone();
    let tamanho_torneio = competidores.len();
    for i in 1..tamanho_torneio {
        if competidores[i].resultado > campeao_atual.resultado {
            campeao_atual = competidores[i].clone();
        }
    }

    campeao_atual
}

fn selecao(populacao: Vec<Solucao>) -> Vec<Solucao> {
    // TODO: externalizar tamanho do torneio. Desacoplar
    let tamanho_torneio = 2;
    let mut rng = rand::thread_rng();

    let mut populacao_final: Vec<Solucao> = Vec::new();
    let mut competidores: Vec<Solucao> = Vec::new();

    for _ in 0..populacao.len() {
        for _ in 0..tamanho_torneio {
            let competidor: &Solucao = rng.choose(&populacao).unwrap();
            competidores.push(competidor.clone());
        }
        populacao_final.push(realiza_torneio(competidores.clone()));
        competidores.clear();
    }

    populacao_final
}

pub fn genetico(problema: &Problema, tamanho_populacao: usize) -> Solucao {
    // Cria a população inicial e calcula seu fitness
    let melhor_solucao: Solucao;
    let populacao: Vec<Solucao> = populacao_inicial(tamanho_populacao, problema);

    // while condição de parada
    // (n° x de iterações sem mudar o melhor indivíduo)

    // Seleção
    // seleciona os pais que irão cruzar
    // obs: guardar melhor indivíduo

    // cruzamento (recebe a população de pais escolhidos)
    // -> população de filhos

    // mutação na nova população de filhos e pais
    // (recebe população de pais e filhos)
    // -> população mutada de (filhos, pais)

    // Seleção (recebe população mutada de pais e filhos)
    // seleciona quem irá para a proxima geração
    // nova população

    melhor_solucao = get_melhor_solucao(&populacao);
    melhor_solucao
}
