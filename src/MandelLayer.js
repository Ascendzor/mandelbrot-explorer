import { MapLayer, withLeaflet } from 'react-leaflet'
import L from 'leaflet'
import createMandelbrotImage from './createMandelbrotImage'
import context from './context'

L.MandelbrotLayer = L.GridLayer.extend({
  createTile: (coords, done) => {
    const tile = document.createElement('canvas')
    const tileSize = 256
    tile.width = tile.height = tileSize
    const context = tile.getContext('2d')
    const imgData = context.createImageData(tileSize, tileSize)
    const mandelImage = createMandelbrotImage(imgData, coords)
    context.putImageData(mandelImage, 0, 0)

    setTimeout(() => {
      context.putImageData(mandelImage, 0, 0)
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
