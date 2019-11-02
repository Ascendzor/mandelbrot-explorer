import React from 'react'
import {FaBitcoin} from 'react-icons/fa'

export default ({onClick}) => {
  return <div className={'Share mapButton'} onClick={onClick} style={{
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