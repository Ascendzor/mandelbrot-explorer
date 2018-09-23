import hsvToRgb from 'hsv2rgb'
import createColourScale from './createColourScale'

const maxIterations = 100
const tileSize = 256
const normalize = (val, max, min) => (val-min) / (max-min)
const theMandelbrot = (z, c) => {
  return {
    x: z.x*z.x - z.y*z.y + c.x,
    y: 2 * z.x * z.y + c.y
  }
}
const logBase = 1.0 / Math.log(2.0)
const logHalfBase = Math.log(0.5)*logBase
const colourScale = createColourScale()

export default (imgData, coords) => {
  let exes = []

  const max = (coords.z+1)*4*tileSize

  // const colourScale = createColourScale()

  let counter = 0
  for(let y=0; y<tileSize; y++) for(let x=0; x<tileSize; x++) {
    const offsetX = x+(coords.x*tileSize)-tileSize/2
    const offsetY = y+(-coords.y*tileSize)-tileSize/2

    const normalizedX = normalize(offsetX, max, 0)
    const normalizedY = normalize(offsetY, max, 0)

    // const real = (normalizedX*3.5) - 2.5      //These magic numbers are for the mandelbrot scale
    // const imaginary = (normalizedY*2) - 1     //These magic numbers are for the mandelbrot scale

    const real = normalizedX      //These magic numbers are for the mandelbrot scale 3.5/2
    const imaginary = normalizedY

    let iteration = 0
    let z = {x: real, y: imaginary}
    const c = {x: real, y: imaginary}
    while((z.x**2+z.y**2)<2**2 && iteration<maxIterations) {
      z = theMandelbrot(z, c)
      iteration++
    }
    // const pixel = (((y-offsetY)*tileSize)+(x-offsetX))*4
    const pixel  = (((tileSize-1-y) * tileSize) + x) * 4
    // console.log(pixel)
    if(iteration === maxIterations) {
      imgData.data[pixel+0] = 0
      imgData.data[pixel+1] = 0
      imgData.data[pixel+2] = 0
      imgData.data[pixel+3] = 255
    } else {
      const somethingElse = 5 + iteration - logHalfBase - Math.log(Math.log((z.x*z.x)+(z.y*z.y)))*logBase;
      const somethingElser = hsvToRgb(360.0*somethingElse/2/iteration, 1.0, 1.0)

      // double size = Math.Sqrt(re * re + im * im);
      // double smoothed = Math.Log(Math.Log(size) * ONE_OVER_LOG2) * ONE_OVER_LOG2;
      // int colorI = (int)(Math.Sqrt(i + 1 - smoothed) * gradient.Scale + gradient.Shift) % colors.Length;
      // Color color = colors[colorI];
      const size = Math.sqrt(z.x*z.x + z.y*z.y)
      const smoothed = Math.log(Math.log(size) * 1/Math.log(2)) * 1/Math.log(2)
      const colourLookup = (Math.sqrt(iteration + 1 - smoothed) * 256) + 0

      imgData.data[pixel+0] = colourScale[iteration].r
      imgData.data[pixel+1] = colourScale[iteration].g
      imgData.data[pixel+2] = colourScale[iteration].b
      imgData.data[pixel+3] = colourScale[iteration].a
    }
  }
  return imgData
}
