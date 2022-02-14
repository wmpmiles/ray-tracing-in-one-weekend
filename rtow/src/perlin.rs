use crate::random::Random;
use geometry3d::Point3;
use n_tuple::*;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};

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
pub enum Perlin {
    U(Uninit),
    I(Init),
}

impl Perlin {
    pub fn noise(&mut self, p: Point3) -> f64 {
        if let Perlin::U(u) = self {
            *self = Perlin::I(Perlin::init(u));
        }

        if let Perlin::I(i) = self {
            Self::noise_calc(i, p)
        } else {
            panic!("Failed to initialise Perlin.");
        }
    }

    fn noise_calc(s: &Init, p: Point3) -> f64 {
        let p = NTuple::from(p);
        let int = p.map(|x| x.floor() as i64);
        let dec = p.map(|x| x - x.floor());
        let dec = dec.map(|x| x * x * (3.0 - 2.0 * x));
        let mut c = [[[0.0; 2]; 2]; 2];
        for (i, u) in c.iter_mut().enumerate() {
            for (j, v) in u.iter_mut().enumerate() {
                for (k, w) in v.iter_mut().enumerate() {
                    let offset = ntuple!(i as i64, j as i64, k as i64);
                    let indices =
                        int.combine(offset, |x, y| (x + y).rem_euclid(s.size as i64) as usize);
                    let index = s.perm_x[indices[0]] ^ s.perm_y[indices[1]] ^ s.perm_z[indices[2]];
                    *w = s.ranfloat[index];
                }
            }
        }

        Self::trilinear_interp(c, dec)
    }

    fn trilinear_interp(c: [[[f64; 2]; 2]; 2], dec: NTuple<f64, 3>) -> f64 {
        let mut accum = 0.0;
        for (i, u) in c.iter().enumerate() {
            for (j, v) in u.iter().enumerate() {
                for (k, w) in v.iter().enumerate() {
                    let indices = ntuple!(i, j, k).map(|x| x as f64);
                    let interp = indices.combine(dec, |x, y| x * y + (1.0 - x) * (1.0 - y));
                    accum += interp.fold(*w, |acc, x| acc * x);
                }
            }
        }
        accum
    }

    fn init(u: &Uninit) -> Init {
        let mut rng = Random::new(rand::rngs::StdRng::seed_from_u64(u.seed));
        let size = u.size;
        let ranfloat = Self::generate_ranfloat(u, &mut rng);
        let perm_x = Self::generate_perm(u, &mut rng);
        let perm_y = Self::generate_perm(u, &mut rng);
        let perm_z = Self::generate_perm(u, &mut rng);
        Init {
            size,
            ranfloat,
            perm_x,
            perm_y,
            perm_z,
        }
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
