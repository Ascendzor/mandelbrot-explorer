import React, { Component } from 'react'
import { Map } from 'react-leaflet'
import Leaflet from 'leaflet'
import MandelLayer from './MandelLayer'

const boundsSize = 4096
const bounds = [
  [-boundsSize*3.5, -boundsSize*2],
  [boundsSize*3.5, boundsSize*2]
]

class Mandelbrot extends Component {
  constructor() {
    super()
    // let imgData = context().createImageData(256, 256)
    // context().putImageData(createMandelbrotImage(imgData, 256, 256), 0, 0)
    this.state = {
      center: [0, 0]
    }
  }
  render() {
    const {center, zoom} = this.state
    return <div className="Mandelbrot">
      <Map
        style={{height: '100%'}}
        crs={Leaflet.CRS.Simple}
        bounds={bounds}
        center={center}
        onViewportChange={e => {
          // console.log(e)
        }}
        zoom={zoom}
        minZoom={0}
        tms={true}
        onClick={e => console.log('on click')}
        zoomstart={e => console.log('zoomstart')}
        onMovestart={e => console.log(e.target)}
        onMoveend={e => console.log(e)}

      >
        <MandelLayer
          maxZoom={50}
          tileSize={256}
        />
      </Map>
    </div>
  }
}

export default Mandelbrot
