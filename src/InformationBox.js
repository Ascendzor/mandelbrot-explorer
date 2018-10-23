import React, { Component } from 'react'
import {FaInfo, FaTimesCircle} from 'react-icons/fa'
import Button from '@material-ui/core/Button'
import DeleteIcon from '@material-ui/icons/Delete'
import IconButton from '@material-ui/core/IconButton'

class InformationBox extends Component {
  constructor() {
    super()
  }
  render() {
    const {onClose} = this.props
    return <div className={'InformationBox'} style={{
      width: 400,
      borderRadius: 10,
      padding: 10,
      paddingBottom: 20,
      fontSize: 16,
      color: 'white',
      textAlign: 'center',
      background: 'rgba(40, 40, 40, .94)',
      display: 'relative'
    }}>
      <div>
        <p>Welcome</p>
        <p>This is a map of the Mandelbrot Set. You can interact with this map like you do with Google Maps.</p>
        <p>Pinch, zoom, scroll, drag all work.</p>
        <hr />
        <p>None of the imagery you see here was man-made.</p>
        <p>The mathematical formula that generated this is:</p>
        <p>Z = Z ² + C</p>
        <hr />
        <p>
          How should you use this?
        </p>
        <p style={{fontWeight: 'bold', fontSize: 18}}>
          <span style={{color: '#4A9FF4'}}>Explore </span>
          <span style={{color: '#54AA00'}}>Appreciate </span>
          <span style={{color: '#B20C54'}}>Share</span>
        </p>
        <hr />
        <p>Here are resources where you can learn more</p>
        <p>
          <a className='link' target='_blank' href='https://www.youtube.com/watch?v=DKHucotq6J0&feature=youtu.be&t=69'>How it is calculated mathematically</a>
        </p>
        <p>
          <a className='link' target='_blank' href='https://www.youtube.com/watch?v=DKHucotq6J0&feature=youtu.be&t=69'>Humourous religious description</a>
        </p>
      </div>
      <IconButton onClick={onClose} style={{position: 'absolute', top: 0, right: 0}}>
        <FaTimesCircle style={{color: 'rgba(230, 230, 230, .5)'}}/>
      </IconButton>
    </div>
  }
}

export default InformationBox
