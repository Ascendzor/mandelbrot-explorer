import createColourScale from './createColourScale'
import {cloneDeep} from 'lodash'

const maxIterations = 50
const tileSize = 256
const theMandelbrot = (z, c) => {
  return {
    x: z.x*z.x - z.y*z.y + c.x,
    y: 2 * z.x * z.y + c.y
  }
}
const colourScale = createColourScale()

export default (imgData, coords) => {
  coords = cloneDeep(coords)
  coords.y = coords.y-1
  coords.z = coords.z+1

  const xMin = -((2)**coords.z)
  const xBounds = {min: xMin, max: -xMin/2}

  const yMin = xMin/2
  const yBounds = {min: yMin, max: -yMin}

  for(let y=0; y<tileSize; y++) for(let x=0; x<tileSize; x++) {
    const preNormalizedPixel = coords.x + (x / tileSize)
    const rangePercentile = ((preNormalizedPixel-xBounds.min) * 100) / (xBounds.max - xBounds.min)

    const ypreNormalizedPixel = coords.y + (y / tileSize)
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
    
    const pixel  = (((tileSize-1-y) * tileSize) + x) * 4
    if(iteration === maxIterations*coords.z) {
      imgData.data[pixel+0] = 0
      imgData.data[pixel+1] = 0
      imgData.data[pixel+2] = 0
      imgData.data[pixel+3] = 255
    } else {
      imgData.data[pixel+0] = colourScale[iteration].r
      imgData.data[pixel+1] = colourScale[iteration].g
      imgData.data[pixel+2] = colourScale[iteration].b
      imgData.data[pixel+3] = colourScale[iteration].a
    }
  }
  return imgData
}
