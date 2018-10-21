import React, { Component } from 'react'
// import {PaperPlane} from 'react-icons/paper-plane'
import {FaPaperPlane, FaCamera} from 'react-icons/fa'

class Share extends Component {
  constructor() {
    super()
  }
  render() {
    return <div className={'Share mapButton'} onClick={e => this.props.onClick()} style={{
      width: 80,
      height: 80,
      borderRadius: 10,
      padding: 5,
      paddingTop: 10,
      fontSize: 12,
      color: 'white'
    }}>
      Share
      <FaCamera style={{
        paddingTop: 6,
        width: 50,
        height: 50
      }}/>
    </div>
  }
}

export default Share
