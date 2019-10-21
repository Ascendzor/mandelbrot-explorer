export default async ({coords, xBounds, yBounds, tileSize, maxIterations}) => {
  // const rusty = await import('./rust/pkg');
  // let iterations = []
  // for(let y=0; y<tileSize; y++) for(let x=0; x<tileSize; x++) {
  //   const preNormalizedPixel = coords.x + (x/tileSize)
  //   const rangePercentile = ((preNormalizedPixel-xBounds.min) * 100) / (xBounds.max - xBounds.min)

  //   const ypreNormalizedPixel = coords.y + (y/tileSize)
  //   const yrangePercentile = ((ypreNormalizedPixel-yBounds.min) * 100) / (yBounds.max - yBounds.min)
  //   const real = (rangePercentile * (1 - -2) / 100) + -2
  //   const imaginary = (yrangePercentile * (1 - -1) / 100) + -1

  //   const iteration = rusty.mandelbrot(maxIterations*coords.z, real, imaginary)
  //   iterations.push(iteration)
  // }

  // return {iterations}
  return {iterations: 0}
}