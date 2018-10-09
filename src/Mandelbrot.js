import React, { Component } from 'react'
import { Map } from 'react-leaflet'
import Leaflet from 'leaflet'
import MandelLayer from './MandelLayer'
import Settings from './Settings'
import {tileSize} from './constants'
import Share from './Share'
import toClipboard from 'copy-text-to-clipboard'
import Button from '@material-ui/core/Button'
import {FaClipboardList} from 'react-icons/fa'
import Snackbar from '@material-ui/core/Snackbar'

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
    this.state = {
      viewport: {
        center: [0, -tileSize],
        zoom: 0
      },
      showUrl: true,
      showSnackBar: false,
      timeout: 0
    }

    // setTimeout(() => this.setState({viewport: {
    //   center: [0, -tileSize/2],
    //   zoom: 0
    // }}), 1)
    setTimeout(() => this.setState({viewport: {
      center: [266.6807107763253, -43.019447937294274],
      zoom: 37
    }}), 1)
  }
  render() {
    const {viewport, showUrl, showSnackBar, timeout} = this.state

    console.log(this.state)
    return <div className="Mandelbrot">
      <div style={{
        position: 'absolute',
        zIndex: 1000,
        left: 13,
        top: 100
      }}>
        {<Settings
          viewport={viewport}
        />}
      </div>
      <div style={{
        position: 'absolute',
        right: 13,
        bottom: 13,
        zIndex: 1000
      }}>
        <Share onClick={() => this.setState({showUrl: !this.state.showUrl})}/>
      </div>
      {showUrl && <div className={'clipboardcopy'} style={{
        position: 'absolute',
        marginLeft: 'auto',
        marginRight: 'auto',
        right: 0,
        left: 0,
        top: 200,
        zIndex: 1000,
        width: 240,
        borderRadius: 10,
        color: 'white',
        paddingTop: 10,
        height: 100
      }}>
        <div>
          {window.location.href}
        </div>
        <div onClick={() => {
          let {timeout} = this.state
          const didCopy = toClipboard(window.location.href)

          clearTimeout(timeout)
          timeout = setTimeout(() => {
            this.setState({showSnackBar: false})
          }, 5000)

          this.setState({
            timeout,
            showSnackBar: true
          })
          console.log(didCopy)
        }}>
          <FaClipboardList style={{
            paddingTop: 7,
            width: 60,
            height: 60
          }}/>
        </div>
      </div>}
      <Snackbar
        anchorOrigin={{ vertical: 'top', horizontal: 'center' }}
        open={showSnackBar}
        onClose={() => this.setState({showSnackBar: false})}
        ContentProps={{
          'aria-describedby': 'message-id',
        }}
        width={100}
        message={<span id="message-id">URL copied to Clipboard!</span>}
      />
      <Map
        style={{height: '100%'}}
        crs={Leaflet.CRS.Simple}
        bounds={bounds}
        viewport={viewport}
        onViewportChange={e => {
          this.setState({viewport: e})
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
