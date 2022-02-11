use crate::random::Random;
use n_tuple::NTuple;
use geometry3d::Point3;
use rand::SeedableRng;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Uninit {
    pub size: usize,
    pub seed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Init {
    size: usize,
    ranfloat: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Perlin{
    U(Uninit),
    I(Init),
}

impl Perlin {
    pub fn noise(&mut self, p: Point3) -> f64 {
        if let Perlin::U(u) = self {
            *self = Perlin::I(Perlin::init(u));
        }

        if let Perlin::I(s) = self {
            let p = NTuple::from(p);
            let p = p.map(|x| (4.0 * x.abs()) as usize % s.size);
            let i = s.perm_x[p[0]] ^ s.perm_y[p[1]] ^ s.perm_z[p[2]];
            s.ranfloat[i]
        } else {
            panic!("Failed to initialise Perlin.");
        }
    }

    fn init(u: &Uninit) -> Init{
        let mut rng = Random::new(rand::rngs::StdRng::seed_from_u64(u.seed));
        let size = u.size;
        let ranfloat = Self::generate_ranfloat(u, &mut rng);
        let perm_x = Self::generate_perm(u, &mut rng);
        let perm_y = Self::generate_perm(u, &mut rng);
        let perm_z = Self::generate_perm(u, &mut rng);
        Init { size, ranfloat, perm_x, perm_y, perm_z }
    }

    fn generate_ranfloat(u: &Uninit, rng: &mut Random<rand::rngs::StdRng>) -> Vec<f64> {
        let mut rf = Vec::with_capacity(u.size);
        for _ in 0..u.size {
            rf.push(rng.random());
        }
        rf
    }

    fn generate_perm(u: &Uninit, rng: &mut Random<rand::rngs::StdRng>) -> Vec<usize> {
        let mut p = Vec::with_capacity(u.size);
        for i in 0..u.size {
            p.push(i);
        }
        Self::permute(&mut p, rng);
        p
    }

    fn permute(p: &mut Vec<usize>, rng: &mut Random<rand::rngs::StdRng>) {
        for i in (1..p.len()).rev() {
            let target = rng.random_range(0..i);
            p.swap(i, target);
        }
    }
}
