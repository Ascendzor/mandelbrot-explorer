import createColourScale from './createColourScale'
import {cloneDeep, times, flatten} from 'lodash'
import {tileSize, maxIterations} from './constants'

const theMandelbrot = (z, c) => {
  return {
    x: z.x*z.x - z.y*z.y + c.x,
    y: 2 * z.x * z.y + c.y
  }
}
const colourScale = createColourScale()

export default (imgData, coords, qualityScale) => {
  coords = cloneDeep(coords)
  coords.y = coords.y-1
  coords.z = coords.z+1

  const xMin = -((2)**coords.z)
  const xBounds = {min: xMin, max: -xMin/2}

  const yMin = xMin/2
  const yBounds = {min: yMin, max: -yMin}

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

    let colour = colourScale[iteration]

    const pixel  = (((tileSize-1-y) * tileSize) + x) * 4
    if(iteration === maxIterations*coords.z*qualityScale) {
      imgData.data[pixel+0] = 0
      imgData.data[pixel+1] = 0
      imgData.data[pixel+2] = 0
      imgData.data[pixel+3] = 255
    } else {
      imgData.data[pixel+0] = colour.r
      imgData.data[pixel+1] = colour.g
      imgData.data[pixel+2] = colour.b
      imgData.data[pixel+3] = colour.a
    }
  }
  return imgData
}
