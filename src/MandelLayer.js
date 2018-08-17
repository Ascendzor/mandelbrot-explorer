import React from 'react'
import { MapLayer, withLeaflet } from 'react-leaflet'
import L from 'leaflet'
import createMandelbrotImage from './createMandelbrotImage'
import context from './context'

L.TileLayer.MandelbrotLayer = L.TileLayer.extend({
    createTile: (coords, done) => {
      console.log('creating tile')
      // return document.createElement('div').appendChild(document.createTextNode('hi'))
      // // console.log(coords)
      // // const canvas = document.createElement('canvas')
      // // const ctx = canvas.getContext('2d')
      // //
      // // let imgData = ctx.createImageData(256, 256)
      // // canvas.width = imgData.width
      // // canvas.height = imgData.height
      // // imgData = createMandelbrotImage(imgData, 256, 256)
      // // // ctx.putImageData(imgData, 0, 0)
      // // console.log(canvas)
      // // ctx.fillStyle = 'black'
      // // ctx.fillRect(0, 0, 50, 50)
      // // return canvas
      var tile = document.createElement('canvas');

      var tileSize = 256;
      tile.setAttribute('width', tileSize.x);
      tile.setAttribute('height', tileSize.y);

      var ctx = tile.getContext('2d');

      // Draw whatever is needed in the canvas context
      // For example, circles which get bigger as we zoom in
      ctx.beginPath();
      ctx.arc(tileSize.x/2, tileSize.x/2, 4 + coords.z*4, 0, 2*Math.PI, false);
      ctx.fill();

      return tile;
    }
})

class MandelLayer extends MapLayer {
  createLeafletElement(a, b, c) {
    console.log(a, b, c)
    return new L.TileLayer.MandelbrotLayer(a, b, c)
  }
}

export default withLeaflet(MandelLayer)
