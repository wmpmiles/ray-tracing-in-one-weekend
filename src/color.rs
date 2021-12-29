use crate::vec3::Vec3;
pub type Color = Vec3;

fn map_byte(val: f64) -> u8 {
    let val = val.clamp(0.0, 1.0);
    const RATIO: f64 = 255.999;
    (RATIO * val) as u8
}

impl Color {
    pub fn write(self, samples_per_pixel: u32, data: &mut Vec<u8>) {
        let scale = 1.0 / samples_per_pixel as f64;
        let color = self.elementwise(|x| (scale * x).sqrt());

        data.push(map_byte(color.0));
        data.push(map_byte(color.1));
        data.push(map_byte(color.2));
        data.push(255);
    }
}
