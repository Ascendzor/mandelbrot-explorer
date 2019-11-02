import React, { Component } from 'react'
import {FaBitcoin} from 'react-icons/fa'

class Info extends Component {
  constructor() {
    super()
  }
  render() {
    return <div className={'Share'} onClick={e => this.props.onClick()} style={{
      width: 40,
      height: 40,
      borderRadius: 10,
      padding: 10,
      paddingTop: 6,
      fontSize: 12,
      color: 'white'
    }}>
      Info
      <FaBitcoin style={{
        paddingTop: 1,
        width: 30,
        height: 30
      }}/>
    </div>
  }
}

export default Info
