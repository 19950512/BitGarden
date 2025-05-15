use noise::{NoiseFn, Perlin};
use rand::Rng;
use std::fmt;

use std::fs::File;
use std::io::Write;

const LARGURA: usize = 120;
const ALTURA: usize = 120;
const ESCALA: f64 = 20.0;

#[derive(Clone, Copy)]
enum Terreno {
    Agua,
    Areia,
    Grama,
    Floresta,
    Montanha,
}

impl fmt::Display for Terreno {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let simbolo = match self {
            Terreno::Agua => '~',
            Terreno::Areia => '.',
            Terreno::Grama => ',',
            Terreno::Floresta => 'T',
            Terreno::Montanha => '^',
        };
        write!(f, "{}", simbolo)
    }
}

fn tipo_terreno(valor: f64) -> Terreno {
    if valor < -0.2 {
        Terreno::Agua
    } else if valor < 0.0 {
        Terreno::Areia
    } else if valor < 0.3 {
        Terreno::Grama
    } else if valor < 0.6 {
        Terreno::Floresta
    } else {
        Terreno::Montanha
    }
}

fn gerar_mapa() -> Vec<Vec<Terreno>> {
    let mut rng = rand::thread_rng();

    let seed: u32 = rng.gen_range(0..1_000_000);

    println!("Semente do mapa: {}", seed);

    let perlin = Perlin::new(seed);
    let mut mapa = vec![vec![Terreno::Agua; LARGURA]; ALTURA];

    for y in 0..ALTURA {
        for x in 0..LARGURA {
            let nx = x as f64 / ESCALA;
            let ny = y as f64 / ESCALA;
            let ruido = perlin.get([nx, ny]);
            mapa[y][x] = tipo_terreno(ruido);
        }
    }

    mapa
}

fn imprimir_mapa(mapa: &[Vec<Terreno>]) {
    for linha in mapa {
        for terreno in linha {
            print!("{}", terreno);
        }
        println!();
    }
}


fn salvar_mapa(mapa: &[Vec<Terreno>], caminho: &str) {
    let mut arquivo = File::create(caminho).expect("Erro ao criar arquivo");
    for linha in mapa {
        for terreno in linha {
            write!(arquivo, "{}", terreno).unwrap();
        }
        writeln!(arquivo).unwrap();
    }
}


fn main() {
    let mapa = gerar_mapa();
    imprimir_mapa(&mapa);
    salvar_mapa(&mapa, "mapa.txt");
}
