/* eslint-disable */

// importScripts('./rust/pkg');
fetch('./rust/target/wasm32-unknown-unknown/release/PROJECT.wasm').then(response => response.arrayBuffer()).then(bytes => WebAssembly.instantiate(bytes)).then(obj => console.log("succeeded: " + obj.instance.exports.add(1, 2))).catch(error => console.log("error: " + error));

// import("./rust/pkg").then(rusty => {
//   console.log(rusty)
//   onmessage = evt => {
//     console.log(evt.data)
//     const [coords, xBounds, yBounds, tileSize, maxIterations] = evt.data
//     console.log({rusty})
//     let iterations = []
//     // for(let y=0; y<tileSize; y++) for(let x=0; x<tileSize; x++) {
//     //   const preNormalizedPixel = coords.x + (x/tileSize)
//     //   const rangePercentile = ((preNormalizedPixel-xBounds.min) * 100) / (xBounds.max - xBounds.min)

//     //   const ypreNormalizedPixel = coords.y + (y/tileSize)
//     //   const yrangePercentile = ((ypreNormalizedPixel-yBounds.min) * 100) / (yBounds.max - yBounds.min)
//     //   const real = (rangePercentile * (1 - -2) / 100) + -2
//     //   const imaginary = (yrangePercentile * (1 - -1) / 100) + -1

//     //   const iteration = self.rusty.mandelbrot(maxIterations*coords.z, real, imaginary)
//     //   console.log('pushing')
//     //   iterations.push(iteration)
//     // }
//     console.log(iterations)
//     self.postMessage({iterations, coords})
//   }
// })

onmessage = async evt => {
  // import("./rust/pkg").then(console.log)
  console.log({ thing });
  console.log('on message');
};
