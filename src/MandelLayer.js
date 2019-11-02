import { MapLayer, withLeaflet } from 'react-leaflet'
import L from 'leaflet'
import {tileSize} from './constants'
import {renderTile} from './WorkerManager'

L.MandelbrotLayer = L.GridLayer.extend({
  createTile: (coords, done) => {
    const tile = document.createElement('canvas')
    tile.width = tile.height = tileSize
    const context = tile.getContext('2d')

    // if(coords.x !== 0 || coords.y !== 0) return tile
    renderTile({coords, computeOption: 'rust'}).then(imageData => {
      context.putImageData(imageData, 0, 0)
      done(null, tile)
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
