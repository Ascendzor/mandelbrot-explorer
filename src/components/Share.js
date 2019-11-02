import React from 'react'
import {FaCamera} from 'react-icons/fa'

export default ({onClick}) => {
  return <div className={'Share mapButton'} onClick={onClick} style={{
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