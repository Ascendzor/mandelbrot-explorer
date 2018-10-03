import React, { Component } from 'react'
import Paper from '@material-ui/core/Paper'
import Slider from '@material-ui/lab/Slider'

class Mandelbrot extends Component {
  constructor() {
    super()
    this.state = {
      quality: 20
    }
  }
  render() {
    const {quality} = this.state



    return <Paper
      style={{
        padding: 15,
        width: 200
      }}
      elevation={6}
    >
      <div>
        <div>settings</div>
        <div style={{paddingRight: 15}}>
          <Slider
            value={quality}
            onChange={(event, value) => this.setState({quality: value})}
          />
        </div>
      </div>
    </Paper>
  }
}

export default Mandelbrot
