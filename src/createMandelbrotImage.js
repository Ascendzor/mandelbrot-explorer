import createColourScale from './createColourScale'
import {tileSize, maxIterations} from './constants'
import {getIterationsForTile} from './TileManager'

const theMandelbrot = (z, c) => {
  return {
    x: z.x*z.x - z.y*z.y + c.x,
    y: 2 * z.x * z.y + c.y
  }
}
const colourScale = createColourScale()

const renderIterationsIntoPixels = ({imgData, iterations, tileCoords}) => {
  for(let y=0; y<tileSize; y++) for(let x=0; x<tileSize; x++) {
    const {z} = tileCoords
    const pixel  = (((tileSize-1-y) * tileSize) + x)
    const colour = colourScale[iterations[pixel]]

    if(iterations[pixel] === maxIterations*tileCoords.z) {
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

export default (context, tileCoords, qualityScale) => {
  const imgData = context.createImageData(tileSize, tileSize)
  tileCoords = {
    ...tileCoords,
    y: tileCoords.y-1,
    z: tileCoords.z+1
  }

  const xMin = -((2)**tileCoords.z)
  const xBounds = {min: xMin, max: -xMin/2}

  const yMin = xMin/2
  const yBounds = {min: yMin, max: -yMin}
  return getIterationsForTile({tileCoords, xBounds, yBounds, tileSize, maxIterations}).then(iterations => {
    return renderIterationsIntoPixels({
      iterations,
      tileCoords,
      imgData
    })
  })
}
