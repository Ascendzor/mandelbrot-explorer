import React, { Component } from 'react'
import { Map } from 'react-leaflet'
import Leaflet from 'leaflet'
import MandelLayer from './MandelLayer'
import Paper from '@material-ui/core/Paper'
import Input from '@material-ui/core/Input'
import Button from '@material-ui/core/Button'
import Settings from './Settings'
import {tileSize} from './constants'

const boundsSize = 4096
const bounds = [
  [-boundsSize*3.5, -boundsSize*2],
  [boundsSize*3.5, boundsSize*2]
]

// const bounds = [
//   [10, 10], [10, 10]
// ]

class Mandelbrot extends Component {
  constructor() {
    super()
    // let imgData = context().createImageData(256, 256)
    // context().putImageData(createMandelbrotImage(imgData, 256, 256), 0, 0)
    this.state = {
      viewport: {
        center: [0, -tileSize],
        zoom: 0
      }
    }

    localStorage.setItem('qualityScale', 1)
    setTimeout(() => this.setState({viewport: {
      center: [0, -tileSize/2],
      zoom: 0
    }}), 1)
  }
  render() {
    const {viewport} = this.state
    return <div className="Mandelbrot">
      <div style={{
        position: 'absolute',
        zIndex: 1000,
        left: 13,
        top: 100
      }}>
        <Settings />
      </div>
      <Map
        style={{height: '100%'}}
        crs={Leaflet.CRS.Simple}
        bounds={bounds}
        viewport={viewport}
        onViewportChange={e => {
          // console.log(e)
        }}
        minZoom={0}
        tms={true}
        onClick={e => console.log('on click')}
        zoomstart={e => console.log('zoomstart')}
        onMovestart={e => console.log(e.target)}
        onMoveend={e => console.log(e)}

      >
        <MandelLayer
          maxZoom={100}
          tileSize={tileSize}
        />
      </Map>
    </div>
  }
}

export default Mandelbrot
