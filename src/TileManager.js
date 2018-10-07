import TileCreator from './TileCreator.worker'
import PubSub from 'pubsub-js'
import {isEqual} from 'lodash'
import createColourScale from './createColourScale'
import GPU from 'gpu.js'
const gpu = new GPU()

const colourScale = createColourScale()

window.tileCreator = new TileCreator()
window.tileCreator.addEventListener('message', event => {
  const {coords, iterations} = event.data
  PubSub.publish('tileLoaded', event.data)
})

const theMandelbrot = (zx, zy, cx, cy) => {
  return [
    zx*zx - zy*zy + cx,
    2 * zx * zy + cy
  ]
}

export const getIterationsForTile = ({coords, xBounds, yBounds, tileSize, maxIterations}) => new Promise((resolve, reject) => {
  // window.tileCreator.postMessage({coords, xBounds, yBounds, tileSize, maxIterations})
  // PubSub.subscribe('tileLoaded', (msg, data) => {
  //   if(isEqual(data.coords, coords)) {
  //     resolve(data.iterations)
  //   }
  // })

  const render = gpu.createKernel(() => {

    const preNormalizedPixel = coords.x + (this.thread.x/tileSize)
    const rangePercentile = ((preNormalizedPixel-xBounds.min) * 100) / (xBounds.max - xBounds.min)

    const ypreNormalizedPixel = coords.y + (this.thread.y/tileSize)
    const yrangePercentile = ((ypreNormalizedPixel-yBounds.min) * 100) / (yBounds.max - yBounds.min)
    const real = (rangePercentile * (1 - -2) / 100) + -2
    const imaginary = (yrangePercentile * (1 - -1) / 100) + -1

    let iteration = 0
    const cx = real
    const cy = imaginary
    let zx = real
    let zy = imaginary
    let zs = [real, imaginary]
    while((zx**2+zy**2)<2**2 && iteration<maxIterations*coords.z) {
      zs = theMandelbrot(zs[0], zs[1], cx, cy)
      iteration++
    }

    const colour = colourScale[iteration]
    console.log('inside the gpu')

    if(iteration === maxIterations*coords.z) {
      this.color(0, 0, 0, 255)
    } else {
      this.color(colour.r, colour.g, colour.b, colour.a)
    }
  }).setOutput([256, 256])
    .setGraphical(true)

  render()

  console.log(render.getCanvas())
})
