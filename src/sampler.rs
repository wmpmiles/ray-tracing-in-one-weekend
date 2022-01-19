pub struct SquareSampler {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    n: u32,
    n2: u32,
    sample: u32,
}

impl SquareSampler {
    pub fn new(x: u32, y: u32, width: u32, height: u32, n: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            n,
            n2: n * n,
            sample: 0,
        }
    }
}

// CP 55 FC
impl Iterator for SquareSampler {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.sample == self.n2 {
            None
        } else {
            // this creates a subpixel bias to the bottom left
            let i = (self.sample % self.n) as f64 / self.n as f64;
            let j = (self.sample / self.n) as f64;

            self.sample += 1;

            // this creates another small bias
            let u = (self.x as f64 + i) / self.width as f64;
            let v = (self.y as f64 + j) / self.height as f64;

            Some((u, v))
        }
    }
}
