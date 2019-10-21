extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(start)]
pub fn run() {
    console::log_1(&"rust wasm run".into());
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
pub fn mandelbrot(max_iter: u32, real: f64, imaginary: f64) -> u32 {
    let mut zrzi = (real, imaginary);
    let mut iter = 0;

    // const theMandelbrot = (z, c) => {
    //     return {
    //         x: z.x*z.x - z.y*z.y + c.x,
    //         y: 2 * z.x * z.y + c.y
    //     }
    // }
    // while((z.x**2+z.y**2)<2**2 && iteration<maxIterations*coords.z) {
    //   z = theMandelbrot(z, c)
    //   iteration++
    // }
    
    while ((zrzi.0*zrzi.0) + (zrzi.1*zrzi.1) <= 4.0) && (iter < max_iter) {
        zrzi = (
            ((zrzi.0*zrzi.0) - (zrzi.1*zrzi.1) + real),
            ((2.0 * zrzi.0 * zrzi.1) + imaginary)
        );
        iter = iter + 1;
    }

    // while (z.norm_sqr() <= 4.0) && (iter < max_iter) {
    //     z = c + (z * z);
    //     iter = iter + 1;
    // }
    iter
}

#[test]
pub fn mandy() {
    println("hola");
}