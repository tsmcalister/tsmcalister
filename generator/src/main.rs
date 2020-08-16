use std::fs::File;
use noise::{NoiseFn, Fbm};
use gif::{Encoder, Frame};
use hsl::HSL;

const RADIUS: f64 = 0.05;
const MAX_X: usize = 512;
const MAX_Y: usize = 512;
const CHANNELS: usize = 4;
const FRAMES: usize = 128;
const SCALE: f64 = 0.5;


pub fn get_noise_loop() -> Vec<Vec<u8>>{
    // returns a continuous 2d fbm noise loop by tracing a 4d "circle"

    let Fbm = Fbm::new();

    let mut min_val = 0.;
    let mut max_val = 0.;

    let mut vals = vec![vec![0u8; MAX_Y * MAX_X * CHANNELS]; FRAMES];

    let DELTA_ANGLE = 2.*std::f64::consts::PI/(FRAMES as f64);

    for t in 0..FRAMES{
        println!("{}", t);
        for y in 0..MAX_Y{
            for x in 0..MAX_X{

                let angle = DELTA_ANGLE*(t as f64);
                let z = RADIUS*angle.cos();
                let w = RADIUS*angle.sin();
                let x_x = (x as f64)/(MAX_X as f64)*SCALE;
                let y_y = (y as f64)/(MAX_Y as f64)*SCALE;
                let q_x = Fbm.get([x_x, y_y, z, w]);
                let q_y = Fbm.get([x_x, y_y, z, w]);
                let r_x = Fbm.get([x_x + 4.0*q_x + 1.7,y_y + 4.0*q_y+9.2,z, w]);
                let r_y = Fbm.get([x_x + 4.0*q_x + 8.3,y_y + 4.0*q_y+2.8,z, w]);
                let val = Fbm.get([x_x+4.0*r_x,y_y+4.0*r_y,z,w]);
                let cols_hsl = HSL {h: val*255., s: 255., l: 255.};
                let (r, g, b) = cols_hsl.to_rgb();


                if ((x as i64)-(MAX_X as i64)/2).pow(2) + ((y as i64)- (MAX_Y as i64)/2).pow(2) < ((MAX_X as i64)/2).pow(2) {
                    vals[t][x*CHANNELS+y*CHANNELS*MAX_X+0] = r;
                    vals[t][x*CHANNELS+y*CHANNELS*MAX_X+1] = g;
                    vals[t][x*CHANNELS+y*CHANNELS*MAX_X+2] = b;
                    vals[t][x*CHANNELS+y*CHANNELS*MAX_X+3] = 255;
                } else {
                    vals[t][x*CHANNELS+y*CHANNELS*MAX_X+2] = 0;
                    vals[t][x*CHANNELS+y*CHANNELS*MAX_X+0] = 0;
                    vals[t][x*CHANNELS+y*CHANNELS*MAX_X+1] = 0;
                    vals[t][x*CHANNELS+y*CHANNELS*MAX_X+2] = 0;
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
    for t in 0..FRAMES{
        let frame = gif::Frame::from_rgba(MAX_X as u16, MAX_Y as u16, &mut *frames[t]);
        encoder.write_frame(&frame).unwrap();
    }
    println!("done");

}
