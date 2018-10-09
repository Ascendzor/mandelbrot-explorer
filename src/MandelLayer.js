import { MapLayer, withLeaflet } from 'react-leaflet'
import L from 'leaflet'
import createMandelbrotImage from './createMandelbrotImage'
import {tileSize} from './constants'
import {cloneDeep} from 'lodash'

L.MandelbrotLayer = L.GridLayer.extend({
  createTile: (coords, done) => {
    coords.y = coords.y+1
    const tile = document.createElement('canvas')
    tile.width = tile.height = tileSize
    const context = tile.getContext('2d')

    createMandelbrotImage(context, coords, localStorage.getItem('qualityScale')).then(mandelbrotImage => {
      setTimeout(() => {
        context.putImageData(mandelbrotImage, 0, 0)
        context.font = "10px Arial"
        context.fillStyle = 'white'
        context.fillText(coords.x + ', ' + coords.y, 10, 40)
        done(null, tile)
      }, 16)
    })
    return tile
  }
})

class MandelLayer extends MapLayer {
  createLeafletElement(a) {
    return new L.MandelbrotLayer(a)
  }
}

export default withLeaflet(MandelLayer)
