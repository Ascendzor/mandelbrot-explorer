import React, { Component } from 'react'
import { Map } from 'react-leaflet'
import Leaflet from 'leaflet'
import MandelLayer from './MandelLayer'
import Settings from './Settings'
import {tileSize} from './constants'
import Share from './Share'
import Donate from './Donate'
import Information from './Information'
import toClipboard from 'copy-text-to-clipboard/dist'
import Button from '@material-ui/core/Button'
import {FaClipboardList} from 'react-icons/fa'
import Snackbar from '@material-ui/core/Snackbar'
import InformationBox from './InformationBox'

const bitcoinAddress = '12psUNxtiCdE26y6DH7hje3bRHwUBeTyaz'

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

    const hashContents = window.location.hash.split('#')[1]
    let viewport = {
      center: [0, -tileSize],
      zoom: 0
    }
    if(hashContents) {
      const datums = decodeURIComponent(hashContents).split('₿')
      setTimeout(() => {
        try {
          this.setState({viewport: {
            center: JSON.parse(datums[0]),
            zoom: datums[1]
          }})
        } catch(error) {
          console.log(error)
        }
      }, 1)
    }

    this.state = {
      viewport,
      showSnackBar: false,
      snackbarMessage: '',
      timeout: 0,
      ShouldShowInformationBox: true
    }
  }
  render() {
    const {viewport, showSnackBar, timeout, snackbarMessage, ShouldShowInformationBox} = this.state

    const hash = {
      center: viewport.center,
      zoom: viewport.zoom
    }

    window.location.hash = JSON.stringify(viewport.center)+'₿'+viewport.zoom
    return <div className="Mandelbrot">
      {ShouldShowInformationBox && <div style={{position: 'absolute', left: 'calc(50% - 200px)', top: 50, zIndex: 1000}}>
        <InformationBox onClose={() => this.setState({ShouldShowInformationBox: false})}/>
      </div>}
      <div style={{
        position: 'absolute',
        right: 17,
        bottom: 17,
        zIndex: 1000
      }}>
        <div style={{display: 'inline-block', verticalAlign: 'bottom', marginRight: 10}}>
          <Donate onClick={() => {
            let {timeout} = this.state

            this.setState({showDonation: !this.state.showDonation, showUrl: false})

            toClipboard(bitcoinAddress)
            clearTimeout(timeout)
            timeout = setTimeout(() => {
              this.setState({showSnackBar: false})
            }, 5000)

            this.setState({
              timeout,
              showSnackBar: true,
              snackbarMessage: 'BTC address copied to clipboard. Thank you.'
            })
          }} />
        </div>
        <div style={{display: 'inline-block', verticalAlign: 'bottom', marginRight: 10}}>
          <Information onClick={() => {
            const {ShouldShowInformationBox} = this.state
            this.setState({
              ShouldShowInformationBox: !ShouldShowInformationBox
            })
          }} />
        </div>
        <div style={{display: 'inline-block', verticalAlign: 'bottom'}}>
          <Share onClick={() => {
            let {timeout} = this.state

            toClipboard(decodeURIComponent(window.location.href))
            clearTimeout(timeout)
            timeout = setTimeout(() => {
              this.setState({showSnackBar: false})
            }, 5000)

            this.setState({
              timeout,
              showSnackBar: true,
              snackbarMessage: 'Link copied to clipboard. Show people what you\'ve found!'
            })
          }} />
        </div>
      </div>
      <Snackbar
        anchorOrigin={{ vertical: 'top', horizontal: 'center' }}
        open={showSnackBar}
        onClose={() => this.setState({showSnackBar: false})}
        ContentProps={{
          'aria-describedby': 'message-id',
        }}
        width={100}
        message={<span>{snackbarMessage}</span>}
      />
      <Map
        style={{height: '100%'}}
        crs={Leaflet.CRS.Simple}
        bounds={bounds}
        viewport={viewport}
        onViewportChange={e => {
          this.setState({viewport: e})
        }}
        onZoomStart={() => {
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
