import React, { Component } from 'react'
// import {PaperPlane} from 'react-icons/paper-plane'
import {FaPaperPlane, FaClipboardList} from 'react-icons/fa'

class Share extends Component {
  constructor() {
    super()
  }
  render() {
    return <div className={'Share'} onClick={e => this.props.onClick()} style={{
      width: 80,
      height: 80,
      borderRadius: 10,
      padding: 10,
      fontSize: 12,
      color: 'white'
    }}>
      Share by URL
      <FaPaperPlane style={{
        paddingTop: 7,
        width: 60,
        height: 60
      }}/>
    </div>
  }
}

export default Share
