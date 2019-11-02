import React from 'react'
import {FaInfo} from 'react-icons/fa'

export default ({onClick}) => {
  return <div className={'Information mapButton'} onClick={onClick} style={{
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