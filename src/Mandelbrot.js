import React, { Component } from 'react'
import { Map, MapLayer, withLeaflet, TileLayer } from 'react-leaflet'
import Leaflet from 'leaflet'
import MandelLayer from './MandelLayer'
import createMandelbrotImage from './createMandelbrotImage'
import context from './context'

console.log(MandelLayer)

class Mandelbrot extends Component {
  componentDidMount() {
    // let imgData = context().createImageData(256, 256)
    // context().putImageData(createMandelbrotImage(imgData, 256, 256), 0, 0)
  }
  render() {
    console.log(MandelLayer)
    return <div className="Mandelbrot">
      <Map
        style={{height: 400}}
        crs={Leaflet.CRS.Simple}
        continuousWorld={true}
        bounds={[[0, 0], [1000, 1000]]}
      >
        <MandelLayer
          minZoom={0}
          maxZoom={25}
          bounds={[[0, 0], [1000, 1000]]}
          continuousWorld={true}
          noWrap={true}
          tileSize={250}
          crs={Leaflet.CRS.Simple}
          detectRetina={false}
          onTileloadstart={e => {
            console.log(e)
          }}
          onTileload={e => {
            console.log(e)
          }}
        />
      </Map>
      <canvas id={'mandelbrot'} width={256} height={256} />
    </div>
  }
}

export default Mandelbrot
