import { MapLayer, withLeaflet } from 'react-leaflet'
import L from 'leaflet'
import createMandelbrotImage from './createMandelbrotImage'

L.MandelbrotLayer = L.GridLayer.extend({
  createTile: (coords, done) => {
    coords.y = -coords.y
    const tile = document.createElement('canvas')
    const tileSize = 256
    tile.width = tile.height = tileSize
    const context = tile.getContext('2d')
    const imgData = context.createImageData(tileSize, tileSize)
    const mandelImage = createMandelbrotImage(imgData, coords)

    setTimeout(() => {
      context.putImageData(mandelImage, 0, 0)
      context.font = "30px Arial"
      context.fillStyle = 'white'
      context.fillText(coords.x + ', ' + coords.y, 10, 40)
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
