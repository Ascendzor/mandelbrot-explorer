extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::console;
use web_sys::{CanvasRenderingContext2d, ImageData};

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

// #[wasm_bindgen]
// pub fn draw(
//     ctx: &CanvasRenderingContext2d,
//     width: u32,
//     height: u32,
//     real: f64,
//     imaginary: f64,
// ) -> Result<(), JsValue> {
//     // The real workhorse of this algorithm, generating pixel data
//     let c = Complex { real, imaginary };
//     let mut data = get_julia_set(width, height, c);
//     let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
//     ctx.put_image_data(&data, 0.0, 0.0)
// }

#[wasm_bindgen]
// pub fn mandelbrot(xCoord: u32, yCoord: u32, max_iter: u32, maxXBounds: u32, minXBounds: u32, maxYBounds: u32, minYBounds: u32)  -> [u32; 256*256] {
pub fn mandelbrot(context: &CanvasRenderingContext2d, xCoord: u32, yCoord: u32, zCoord: u32) -> Result<(), JsValue> {
    // let iterations: [u32; 256*256] = [0; 256*256];
    // context.createImageData(256, 256)

    // let mut zrzi = (real, imaginary);
    // let mut iter = 0;
    
    // while ((zrzi.0*zrzi.0) + (zrzi.1*zrzi.1) <= 4.0) && (iter < max_iter) {
    //     zrzi = (
    //         ((zrzi.0*zrzi.0) - (zrzi.1*zrzi.1) + real),
    //         ((2.0 * zrzi.0 * zrzi.1) + imaginary)
    //     );
    //     iter = iter + 1;
    // }

    // iter
    let mut data = Vec::new();
    for x in 0..256 {
        for y in 0..256 {
            data.push(0 as u8);
            data.push(0 as u8);
            data.push(0 as u8);
            data.push(255 as u8);
        }
    }

    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), 256, 256)?;
    context.put_image_data(&data, 0.0, 0.0)

    // let response = str::from_utf8(&iterations).unwrap();
}

#[test]
pub fn mandy() {
    println("hola");
}