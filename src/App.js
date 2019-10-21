import React, { Component } from 'react';
import './App.css';
import Mandelbrot from './Mandelbrot'

class App extends Component {
  componentDidMount() {
    (async () => {
      window.rusty = await import('./rust/pkg')
      this.forceUpdate()
    })()
  }
  render() {
    return (
      <div className="App">
        {window.rusty && <Mandelbrot />}
      </div>
    );
  }
}

export default App;
