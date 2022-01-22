use n_tuple::*;

/* Behaviours:
 * - Create and access r, g, b
 * - From FloatRgb
 */
#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct Rgb(NTuple<u8, 3>);

impl Rgb {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self(ntuple!(red, green, blue))
    }

    pub fn r(self) -> u8 {
        self.0[0]
    }

    pub fn g(self) -> u8 {
        self.0[1]
    }

    pub fn b(self) -> u8 {
        self.0[2]
    }
}

impl std::convert::From<FloatRgb> for Rgb {
    fn from(frgb: FloatRgb) -> Self {
        Self(frgb.0.map(|x| (x * 256.0) as u8))
    }
}

/* Behaviours:
 * - Create and access r, g, b
 * - Multiply by scalar or vector attenuation values
 */
#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct FloatRgb(NTuple<f64, 3>);

impl FloatRgb {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self(ntuple!(red, green, blue))
    }

    pub fn r(self) -> f64 {
        self.0[0]
    }

    pub fn g(self) -> f64 {
        self.0[1]
    }

    pub fn b(self) -> f64 {
        self.0[2]
    }

    pub fn mix(self, rhs: Self, t: f64) -> Self {
        let s = 1.0 - t;
        Self(self.0.combine(rhs.0, |x, y| t * x + s * y))
    }
}

impl std::ops::Mul<FloatRgb> for FloatRgb {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.combine(rhs.0, |x, y| x * y))
    }
}

/* Behaviours
 * - Create empty accumulator
 * - Add FRGB values to accumulator
 * - Calculate averaged FRGB value
 */
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct FRgbAccumulator {
    sum: NTuple<f64, 3>,
    count: u32,
}

impl FRgbAccumulator {
    pub fn new() -> Self {
        Self {
            sum: ntuple!(0.0, 0.0, 0.0),
            count: 0,
        }
    }

    pub fn average(self) -> FloatRgb {
        FloatRgb(self.sum.map(|x| x / self.count as f64))
    }
}

impl std::ops::AddAssign<FloatRgb> for FRgbAccumulator {
    fn add_assign(&mut self, rhs: FloatRgb) {
        self.sum = self.sum.combine(rhs.0, |x, y| x + y);
        self.count += 1;
    }
}
