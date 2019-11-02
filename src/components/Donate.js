import React, { Component } from 'react'
import {FaBitcoin} from 'react-icons/fa'

class Donate extends Component {
  render() {
    return <div className={'Share mapButton'} onClick={e => this.props.onClick()} style={{
      width: 40,
      height: 40,
      borderRadius: 10,
      padding: 10,
      paddingTop: 6,
      fontSize: 12,
      color: 'white'
    }}>
      Donate
      <FaBitcoin style={{
        paddingTop: 1,
        width: 30,
        height: 30
      }}/>
    </div>
  }
}

export default Donate
