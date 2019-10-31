extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
// use wasm_bindgen::Clamped;
// use web_sys::console;
// use web_sys::{ImageData};

#[wasm_bindgen(start)]
pub fn run() {
    // console::log_1(&"rust wasm run".into());
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
// pub fn mandelbrot(xCoord: u32, yCoord: u32, max_iter: u32, maxXBounds: u32, minXBounds: u32, maxYBounds: u32, minYBounds: u32)  -> [u32; 256*256] {
// pub fn mandelbrot(xCoord: u32, mut yCoord: u32, mut zCoord: u32) -> Result<ImageData, JsValue> {
pub fn mandelbrot(xCoord: i32, mut yCoord: i32, mut zCoord: i32) -> i32 {
    
    // 256 * 256 * 4 = 262144
    let mut imageData: [u8; 262144] = [0; 262144];


    yCoord = -yCoord;
    zCoord = zCoord+1;

    let minXBounds = -((2 as i32).pow(zCoord as u32));
    minXBounds
    // const maxXBounds = -minXBounds/2

    // const minYBounds = minXBounds/2
    // const maxYBounds = -minYBounds

    // let mut data = Vec::new();
    // for x in 0..256 {
    //     for y in 0..256 {
    //         data.push(0 as u8);
    //         data.push(0 as u8);
    //         data.push(0 as u8);
    //         data.push(255 as u8);
    //     }
    // }

    // let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), 256, 256)?;

    // data
}

#[test]
pub fn mandy() {
    println("hola");
}