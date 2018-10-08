import createColourScale from './createColourScale'
import {cloneDeep, times, flatten} from 'lodash'
import {tileSize, maxIterations} from './constants'
import {getIterationsForTile} from './TileManager'

const theMandelbrot = (z, c) => {
  return {
    x: z.x*z.x - z.y*z.y + c.x,
    y: 2 * z.x * z.y + c.y
  }
}
const colourScale = createColourScale()

const renderIterationsIntoPixels = ({imgData, iterations, coords}) => {
  for(let y=0; y<tileSize; y++) for(let x=0; x<tileSize; x++) {
    const {z} = coords
    const pixel  = (((tileSize-1-y) * tileSize) + x)
    const colour = colourScale[iterations[pixel]]

    if(iterations[pixel] === maxIterations*coords.z) {
      imgData.data[pixel*4+0] = 0
      imgData.data[pixel*4+1] = 0
      imgData.data[pixel*4+2] = 0
      imgData.data[pixel*4+3] = 255
    } else {
      imgData.data[pixel*4+0] = colour.r
      imgData.data[pixel*4+1] = colour.g
      imgData.data[pixel*4+2] = colour.b
      imgData.data[pixel*4+3] = colour.a
    }
  }
  return imgData
}

export default (coords, qualityScale) => {
  // const imgData = context.createImageData(tileSize, tileSize)
  coords = cloneDeep(coords)
  coords.y = coords.y-1
  coords.z = coords.z+1

  const xMin = -((2)**coords.z)
  const xBounds = {min: xMin, max: -xMin/2}

  const yMin = xMin/2
  const yBounds = {min: yMin, max: -yMin}

  console.log('before getIterations for eil')
  return getIterationsForTile({coords, xBounds, yBounds, tileSize, maxIterations})
  // return getIterationsForTile({coords, xBounds, yBounds, tileSize, maxIterations}).then(iterations => {
  //   return renderIterationsIntoPixels({
  //     iterations,
  //     coords,
  //     imgData
  //   })
  // })
}
