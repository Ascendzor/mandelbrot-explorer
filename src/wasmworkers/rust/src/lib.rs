extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
// use wasm_bindgen::Clamped;
// use web_sys::{ImageData};

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
    // let tileSize: u16 = 256;
    let tileSize: u16 = 256;

    yCoord = -yCoord;
    zCoord = zCoord+1;

    let minXBounds: f64 = -((2 as i32).pow(zCoord)) as f64;
    let maxXBounds: f64 = -minXBounds/2.0;
    let minYBounds: f64 = minXBounds/2.0;
    let maxYBounds: f64 = -minYBounds;
    // log!("rust: {} {} {} {}", minXBounds, maxXBounds, minYBounds, maxYBounds);

    for y in 0..tileSize {
        for x in 0..tileSize {
            let preNormalizedPixel: f64 = xCoord + ((x as f64)/(tileSize as f64));
            let rangePercentile: f64 = ((preNormalizedPixel-minXBounds) * 100.0) / (maxXBounds - minXBounds);
            
            let ypreNormalizedPixel: f64 = yCoord + ((y as f64)/(tileSize as f64));
            let yrangePercentile: f64 = ((ypreNormalizedPixel-minYBounds) * 100.0) / (maxYBounds - minYBounds);
            
            let imaginary: f64 = (yrangePercentile * (1.0 - -1.0) / 100.0) + -1.0;
            let real: f64 = (rangePercentile * (1.0 - -2.0) / 100.0) + -2.0;

            let pixel: u16 = ((tileSize-1-y) * tileSize) + x;

            let mut zrzi = (real as f64, imaginary as f64);
            let mut iteration = 0;

            while ((zrzi.0*zrzi.0) + (zrzi.1*zrzi.1) <= 4.0) && (iteration < (50*zCoord)) {
                
                zrzi = (
                    ((zrzi.0*zrzi.0) - (zrzi.1*zrzi.1) + real),
                    ((2.0 * zrzi.0 * zrzi.1) + imaginary)
                );
                iteration = iteration + 1;
                if x == 150 && y == 150 && (iteration == 2 || iteration == 159) {
                    log!("rust: iteration: ...");
                    log!("rust: iteration: zrzi: {} {}", zrzi.0, zrzi.1);
                    log!("rust: iteration: crci: {} {}", real, imaginary);
                    log!("rust: iteration: iteration zCoord: {} {}", iteration, zCoord);
                }
            }

            data.push((iteration / 4) as u8);
            data.push((iteration / 2) as u8);
            data.push((iteration) as u8);
            data.push((255) as u8);
        }
    }

    data
}

#[test]
pub fn mandy() {
    println("hola");
}