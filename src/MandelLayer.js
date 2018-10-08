import { MapLayer, withLeaflet } from 'react-leaflet'
import L from 'leaflet'
import createMandelbrotImage from './createMandelbrotImage'
import {tileSize} from './constants'
import {cloneDeep} from 'lodash'

L.MandelbrotLayer = L.GridLayer.extend({
  createTile: (coords, done) => {
    coords.y = coords.y+1
    let tile = document.createElement('canvas')
    tile.width = tile.height = tileSize
    let mandelTile = createMandelbrotImage(coords, localStorage.getItem('qualityScale'))
    tile.getContext('2d').drawImage(mandelTile, 0, 0)

    setTimeout(() => {
      done(null, tile)
    }, 16)
    return tile
  }
})

class MandelLayer extends MapLayer {
  createLeafletElement(a) {
    return new L.MandelbrotLayer(a)
  }
}

export default withLeaflet(MandelLayer)
