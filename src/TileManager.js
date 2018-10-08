import TileCreator from './TileCreator.worker'
import PubSub from 'pubsub-js'
import {isEqual} from 'lodash'
import createColourScale, {redColourScale, greenColourScale, blueColourScale} from './createColourScale'

import GPU from 'gpu.js'
const gpu = new GPU()

const colourScale = createColourScale()

window.tileCreator = new TileCreator()
window.tileCreator.addEventListener('message', event => {
  const {coords, iterations} = event.data
  PubSub.publish('tileLoaded', event.data)
})

export const getIterationsForTile = ({coords, xBounds, yBounds, tileSize, maxIterations}) => {
  // window.tileCreator.postMessage({coords, xBounds, yBounds, tileSize, maxIterations})
  // PubSub.subscribe('tileLoaded', (msg, data) => {
  //   if(isEqual(data.coords, coords)) {
  //     resolve(data.iterations)
  //   }
  // })

  if(coords.x == 0 && coords.y === 0) {
    debugger
  }
  const render = gpu.createKernel(function(options) {
    const coordsX = this.constants.coordsX
    const coordsY = this.constants.coordsY
    const xBoundsMin = this.constants.xBoundsMin
    const xBoundsMax = this.constants.xBoundsMax
    const yBoundsMin = this.constants.yBoundsMin
    const yBoundsMax = this.constants.yBoundsMax
    const tileSize = this.constants.tileSize
    const x = this.thread.x
    const y = this.thread.y

    const preNormalizedPixel = coordsX + (x/tileSize)
    const rangePercentile = ((preNormalizedPixel-xBoundsMin) * 100) / (xBoundsMax - xBoundsMin)

    const ypreNormalizedPixel = coordsY + (y/tileSize)
    const yrangePercentile = ((ypreNormalizedPixel-yBoundsMin) * 100) / (yBoundsMax - yBoundsMin)

    const real = (rangePercentile * 3 / 100) - 2
    const imaginary = (yrangePercentile * 2 / 100) -1

    let iteration = 0
    let zx = real
    let zy = imaginary
    const cx = real
    const cy = imaginary
    let testzx = -1
    while((((zx*zx)+(zy*zy))<4) && iteration<this.constants.maxIterationsWithZ) {
      const tempZx = zx
      zx = (tempZx*tempZx) - (zy*zy) + cx
      zy = (2 * tempZx * zy) + cy
      iteration++
    }
    const r = this.constants.redColourScale[iteration]
    const g = this.constants.greenColourScale[iteration]
    const b = this.constants.blueColourScale[iteration]

    this.color(r, g, b, 1)
  }).setOutput([256, 256]).setGraphical(true).setLoopMaxIterations(10000).setDebug(true).setConstants({
  // }).setOutput([256, 256]).setDebug(true).setLoopMaxIterations(10000).setConstants({
    colourScale,
    redColourScale: redColourScale(),
    greenColourScale: greenColourScale(),
    blueColourScale: blueColourScale(),
    maxIterationsWithZ: maxIterations*coords.z,
    coordsX: coords.x,
    coordsY: coords.y,
    xBoundsMin: xBounds.min,
    xBoundsMax: xBounds.max,
    yBoundsMin: yBounds.min,
    yBoundsMax: yBounds.max,
    tileSize
  })


  const thing = render([0])
  // if(coords.x === 0 && coords.y === 0) {
  //   console.log(thing)
  //   debugger
  // }
  // console.log(thing)
  const canvas = render.getCanvas()
  return canvas
}
