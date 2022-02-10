use crate::config::SamplerConfig;
use crate::image::Image;

pub struct SquareSampler {
    width: u32,
    height: u32,
    n: u32,
    n2: u32,
    pub max_depth: u32,
}

pub struct SquareSamplerIter<'a> {
    x: u32,
    y: u32,
    sample: u32,
    sampler: &'a SquareSampler,
}

impl SquareSampler {
    pub fn new(config: SamplerConfig, image: &Image) -> Self {
        Self {
            width: image.width,
            height: image.height,
            n: config.n,
            n2: config.n * config.n,
            max_depth: config.max_depth,
        }
    }
    
    pub fn samples(&self) -> u32 {
        self.n2
    }

    pub fn iter(&self, x: u32, y: u32) -> SquareSamplerIter {
        SquareSamplerIter {
            x,
            y,
            sample: 0,
            sampler: self,
        }
    }
}

impl Iterator for SquareSamplerIter<'_> {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.sample == self.sampler.n2 {
            None
        } else {
            // this creates a subpixel bias to the bottom left
            let i = (self.sample % self.sampler.n) as f64 / self.sampler.n as f64;
            let j = (self.sample / self.sampler.n2) as f64;

            self.sample += 1;

            // this creates another small bias
            let u = (self.x as f64 + i) / self.sampler.width as f64;
            let v = (self.y as f64 + j) / self.sampler.height as f64;

            Some((u, v))
        }
    }
}
