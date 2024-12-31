use perlin::Perlin;

mod perlin;

fn main() {
    let perlin_noise: Perlin = perlin::Perlin::new(10);
    let noise_value: f32 = perlin_noise.noise_value(2.8, 3.1);
}
