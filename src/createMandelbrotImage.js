import createColourScale from './createColourScale'
import {cloneDeep, times, flatten} from 'lodash'
import {tileSize, maxIterations, antiAliasingSampleSize} from './constants'

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
    let samplePoints = times(antiAliasingSampleSize, aliasX => times(antiAliasingSampleSize, aliasY => {
      return {
        x: x+(aliasX/antiAliasingSampleSize),
        y: y+(aliasY/antiAliasingSampleSize)
      }
    }))
    samplePoints = flatten(samplePoints)

    let samples = []
    let iterations = []
    samplePoints.forEach(samplePoint => {
      const preNormalizedPixel = coords.x + (samplePoint.x / tileSize)
      const rangePercentile = ((preNormalizedPixel-xBounds.min) * 100) / (xBounds.max - xBounds.min)

      const ypreNormalizedPixel = coords.y + (samplePoint.y / tileSize)
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

      samples.push(cloneDeep(colourScale[iteration]))
      iterations.push(iteration)
    })
    const averageIterations = iterations.reduce((sum, iteration) => sum+iteration) / iterations.length

    let colour = samples.reduce((sum, sample) => {
      sum.r += sample.r
      sum.g += sample.g
      sum.b += sample.b
      return sum
    })

    colour.r = colour.r/iterations.length
    colour.g = colour.g/iterations.length
    colour.b = colour.b/iterations.length


    const pixel  = (((tileSize-1-y) * tileSize) + x) * 4
    if(averageIterations === maxIterations*coords.z*qualityScale) {
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
