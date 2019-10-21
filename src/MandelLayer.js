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

    if(coords.x !== 0 || coords.y !== 0) return tile
    createMandelbrotImage(context, coords, localStorage.getItem('qualityScale')).then(mandelbrotImage => {
      setTimeout(() => {
        context.putImageData(mandelbrotImage, 0, 0)
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
