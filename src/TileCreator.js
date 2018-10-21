export default ({coords, xBounds, yBounds, tileSize, maxIterations}) => new Promise((resolve, reject) => {
  const theMandelbrot = (z, c) => {
    return {
      x: z.x*z.x - z.y*z.y + c.x,
      y: 2 * z.x * z.y + c.y
    }
  }

  let iterations = []
  for(let y=0; y<tileSize; y++) for(let x=0; x<tileSize; x++) {
    const preNormalizedPixel = coords.x + (x/tileSize)
    const rangePercentile = ((preNormalizedPixel-xBounds.min) * 100) / (xBounds.max - xBounds.min)

    const ypreNormalizedPixel = coords.y + (y/tileSize)
    const yrangePercentile = ((ypreNormalizedPixel-yBounds.min) * 100) / (yBounds.max - yBounds.min)
    const real = (rangePercentile * (1 - -2) / 100) + -2
    const imaginary = (yrangePercentile * (1 - -1) / 100) + -1

    let iteration = 0
    let z = {x: real, y: imaginary}
    const c = {x: real, y: imaginary}
    while((z.x**2+z.y**2)<2**2 && iteration<maxIterations*coords.z) {
      z = theMandelbrot(z, c)
      iteration++
    }
    iterations.push(iteration)
  }

  resolve({iterations, coords})
})
