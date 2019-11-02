extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub fn mandelbrot(xCoord: f64, mut yCoord: f64, mut zCoord: u32) -> Vec<u8> {
    let mut data = Vec::new();
    let tileSize: u16 = 256;
    
    let mut redColourScale = Vec::new();
    let mut greenColourScale = Vec::new();
    let mut blueColourScale = Vec::new();
    for i in 0..4096 {
        redColourScale.push((i % 256) as u8);
        greenColourScale.push(((i+85) % 256) as u8);
        blueColourScale.push(((i+(85*2)) % 256) as u8);
    }

    let minXBounds: f64 = -((2 as i64).pow(zCoord)) as f64;
    let maxXBounds: f64 = -minXBounds/2.0;
    let minYBounds: f64 = minXBounds/2.0;
    let maxYBounds: f64 = -minYBounds;

    for y in 0..tileSize {
        for x in 0..tileSize {
            let preNormalizedPixel: f64 = xCoord + ((x as f64)/(tileSize as f64));
            let rangePercentile: f64 = ((preNormalizedPixel-minXBounds) * 100.0) / (maxXBounds - minXBounds);
            
            let ypreNormalizedPixel: f64 = yCoord + ((y as f64)/(tileSize as f64));
            let yrangePercentile: f64 = ((ypreNormalizedPixel-minYBounds) * 100.0) / (maxYBounds - minYBounds);
            
            let imaginary: f64 = (yrangePercentile * (1.0 - -1.0) / 100.0) + -1.0;
            let real: f64 = (rangePercentile * (1.0 - -2.0) / 100.0) + -2.0;

            let mut zrzi = (real as f64, imaginary as f64);
            let mut iteration: usize = 0;

            while ((zrzi.0*zrzi.0) + (zrzi.1*zrzi.1) <= 4.0) && (iteration < (50*zCoord) as usize) {
                zrzi = (
                    ((zrzi.0*zrzi.0) - (zrzi.1*zrzi.1) + real),
                    ((2.0 * zrzi.0 * zrzi.1) + imaginary)
                );
                iteration = iteration + 1;
            }

            if iteration == (50*zCoord) as usize {
                data.push(0 as u8);
                data.push(0 as u8);
                data.push(0 as u8);
                data.push((255) as u8);
            } else {
                data.push(redColourScale[iteration]);
                data.push(greenColourScale[iteration]);
                data.push(blueColourScale[iteration]);
                data.push((255) as u8);
            }
            
        }
    }

    data
}

#[test]
pub fn mandy() {
    println("hola");
}