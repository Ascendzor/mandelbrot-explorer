import React, { Component } from 'react'
import { Map } from 'react-leaflet'
import Leaflet from 'leaflet'
import MandelLayer from './MandelLayer'
import createMandelbrotImage from './createMandelbrotImage'
import context from './context'

class Mandelbrot extends Component {
  componentDidMount() {
    // let imgData = context().createImageData(256, 256)
    // context().putImageData(createMandelbrotImage(imgData, 256, 256), 0, 0)
  }
  render() {
    return <div className="Mandelbrot">
      <Map
        style={{height: '100%'}}
        crs={Leaflet.CRS.Simple}
        continuousWorld={true}
        bounds={[[0, 0], [1024, 1024]]}
        tms={true}
      >
        <MandelLayer
          minZoom={0}
          maxZoom={25}
          bounds={[[0, 0], [1000, 1000]]}
          tileSize={256}
        />
      </Map>
    </div>
  }
}

export default Mandelbrot
