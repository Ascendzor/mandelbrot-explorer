import React, { Component } from 'react'
import { Map } from 'react-leaflet'
import Leaflet from 'leaflet'
import MandelLayer from './MandelLayer'
import createMandelbrotImage from './createMandelbrotImage'
import context from './context'

const boundsSize = 4096
const bounds = [
  [-boundsSize*3.5, -boundsSize*2],
  [boundsSize*3.5, boundsSize*2]
]

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
        bounds={bounds}
        center={[0, 0]}
        onViewportChange={e => {
          console.log(e)
        }}
        minZoom={0}
      >
        <MandelLayer
          maxZoom={25}
          tileSize={256}
        />
      </Map>
    </div>
  }
}

export default Mandelbrot
