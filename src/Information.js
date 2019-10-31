import React, { Component } from 'react'
// import {PaperPlane} from 'react-icons/paper-plane'
import {FaInfo} from 'react-icons/fa'

class Information extends Component {
  render() {
    return <div className={'Information mapButton'} onClick={e => this.props.onClick()} style={{
      width: 60,
      height: 60,
      borderRadius: 10,
      padding: 5,
      paddingTop: 5,
      fontSize: 12,
      color: 'white'
    }}>
      Information
      <FaInfo style={{
        paddingTop: 5,
        width: 40,
        height: 40
      }}/>
    </div>
  }
}

export default Information
