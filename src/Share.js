import React, { Component } from 'react'
// import {PaperPlane} from 'react-icons/paper-plane'
import {FaPaperPlane, FaCamera} from 'react-icons/fa'

class Share extends Component {
  constructor() {
    super()
  }
  render() {
    return <div className={'Share'} onClick={e => this.props.onClick()} style={{
      width: 100,
      height: 100,
      borderRadius: 10,
      padding: 10,
      fontSize: 12,
      color: 'white'
    }}>
      Share this view
      <FaCamera style={{
        paddingTop: 16,
        width: 60,
        height: 60
      }}/>
    </div>
  }
}

export default Share
