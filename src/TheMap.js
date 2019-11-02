import React, { useState, useEffect } from 'react'
import { Map } from 'react-leaflet'
import Leaflet from 'leaflet'
import MandelLayer from './MandelLayer'
import {tileSize} from './constants'
import Share from './components/Share'
import Donate from './components/Donate'
import Information from './components/Information'
import toClipboard from 'copy-text-to-clipboard'
import Snackbar from '@material-ui/core/Snackbar'
import InformationBox from './components/InformationBox'

const bitcoinAddress = '12psUNxtiCdE26y6DH7hje3bRHwUBeTyaz'

const boundsSize = 4096
const bounds = [
  [-boundsSize*3.5, -boundsSize*2],
  [boundsSize*3.5, boundsSize*2]
]

export default () => {
  const [viewport, setViewport] = useState({
    center: [0, -tileSize],
    zoom: 0
  })
  const [showSnackBar, setShowSnackBar] = useState(false)
  const [snackbarMessage, setSnackbarMessage] = useState('')
  const [timeoutReference, setTimeoutReference] = useState(0)
  const [ShouldShowInformationBox, setShouldShowInformationBox] = useState(true)

  useEffect(() => {
    const hashState = window.location.hash.split('#')[1]
    if(hashState) {
      const [center, zoom] = decodeURIComponent(hashState).split('₿')
      setViewport({center: JSON.parse(center), zoom})
    }
  }, [])

  useEffect(() => {
    window.location.hash = JSON.stringify(viewport.center)+'₿'+viewport.zoom
  }, [viewport])
  
  return <div className="Mandelbrot">
    {ShouldShowInformationBox && <div style={{position: 'absolute', left: 'calc(50% - 200px)', top: 50, zIndex: 1000}}>
      <InformationBox onClose={() => setShouldShowInformationBox(false)}/>
    </div>}
    <div style={{
      position: 'absolute',
      right: 17,
      bottom: 17,
      zIndex: 1000
    }}>
      <div style={{display: 'inline-block', verticalAlign: 'bottom', marginRight: 10}}>
        <Donate onClick={() => {
          toClipboard(bitcoinAddress)

          clearTimeout(timeoutReference)
          
          setTimeoutReference(setTimeout(() => setShowSnackBar(false), 5000))
          setShowSnackBar(true)
          setSnackbarMessage('BTC address copied to clipboard. Thank you.')
        }} />
      </div>
      <div style={{display: 'inline-block', verticalAlign: 'bottom', marginRight: 10}}>
        <Information onClick={() => setShouldShowInformationBox(!ShouldShowInformationBox)} />
      </div>
      <div style={{display: 'inline-block', verticalAlign: 'bottom'}}>
        <Share onClick={() => {
          toClipboard(decodeURIComponent(window.location.href))
          clearTimeout(timeoutReference)

          setTimeoutReference(setTimeout(() => setShowSnackBar(false), 5000))
          setShowSnackBar(true)
          setSnackbarMessage('Link copied to clipboard. Show people what you\'ve found!')
        }} />
      </div>
    </div>
    <Snackbar
      anchorOrigin={{ vertical: 'top', horizontal: 'center' }}
      open={showSnackBar}
      onClose={() => setShowSnackBar(false)}
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
      onViewportChange={setViewport}
      minZoom={0}
      tms={true}
      // onZoomStart={console}// onZoomStart={console}
      // onClick={e => console.log('on click')}
      // zoomstart={e => console.log('zoomstart')}
      // onMovestart={e => console.log(e.target)}
      // onMoveend={e => console.log(e)}
    >
      <MandelLayer
        maxZoom={100}
        tileSize={tileSize}
      />
    </Map>
  </div>
}