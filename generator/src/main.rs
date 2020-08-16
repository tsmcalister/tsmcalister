use gif::Encoder;
use hsl::HSL;
use noise::{Fbm, NoiseFn};
use std::fs::File;

const RADIUS: f64 = 0.05;
const MAX_X: usize = 256;
const MAX_Y: usize = 256;
const CHANNELS: usize = 4;
const FRAMES: usize = 128;
const SCALE: f64 = 0.5;

/// returns a periodic 2d fbm noise loop (with domain warping) by tracing a 4d "circle"
///
/// TODO: put into a compute shader
fn get_noise_loop() -> Vec<Vec<u8>> {
    let fbm = Fbm::new();

    let mut vals = vec![vec![0u8; MAX_Y * MAX_X * CHANNELS]; FRAMES];

    let delta_angle = 2. * std::f64::consts::PI / (FRAMES as f64);

    for t in 0..FRAMES {
        for y in 0..MAX_Y {
            for x in 0..MAX_X {
                let angle = delta_angle * (t as f64);
                let z = RADIUS * angle.cos();
                let w = RADIUS * angle.sin();

                // warping based on https://www.iquilezles.org/www/articles/warp/warp.htm
                let _x = (x as f64) / (MAX_X as f64) * SCALE;
                let _y = (y as f64) / (MAX_Y as f64) * SCALE;

                let q_x = fbm.get([_x, _y, z, w]);
                let q_y = fbm.get([_x, _y, z, w]);
                let r_x = fbm.get([_x + 4.0 * q_x + 1.7, _y + 4.0 * q_y + 9.2, z, w]);
                let r_y = fbm.get([_x + 4.0 * q_x + 8.3, _y + 4.0 * q_y + 2.8, z, w]);
                let val = fbm.get([_x + 4.0 * r_x, _y + 4.0 * r_y, z, w]);

                let cols_hsl = HSL {
                    h: val * 255.,
                    s: 255.,
                    l: 255.,
                };
                let (r, g, b) = cols_hsl.to_rgb();

                // only draw inside the inscribed circle
                if ((x as i64) - (MAX_X as i64) / 2).pow(2)
                    + ((y as i64) - (MAX_Y as i64) / 2).pow(2)
                    < ((MAX_X as i64) / 2).pow(2)
                {
                    vals[t][x * CHANNELS + y * CHANNELS * MAX_X + 0] = r;
                    vals[t][x * CHANNELS + y * CHANNELS * MAX_X + 1] = g;
                    vals[t][x * CHANNELS + y * CHANNELS * MAX_X + 2] = b;
                    vals[t][x * CHANNELS + y * CHANNELS * MAX_X + 3] = 255;
                }
            }
        }
    }

    vals
}

fn main() {
    let color_map = &[0xFF, 0xFF, 0xFF, 0, 0, 0];
    let mut frames = get_noise_loop();
    let mut image = File::create("out.gif").unwrap();
    let mut encoder = Encoder::new(&mut image, MAX_X as u16, MAX_Y as u16, color_map).unwrap();
    for t in 0..FRAMES {
        let frame = gif::Frame::from_rgba(MAX_X as u16, MAX_Y as u16, &mut *frames[t]);
        encoder.write_frame(&frame).unwrap();
    }
}
