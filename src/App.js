import React, { Component } from 'react';
import logo from './logo.svg';
import './App.css';

import Mandelbrot from './Mandelbrot'

class App extends Component {
  render() {
    return (
      <div className="App">
        <Mandelbrot />
      </div>
    );
  }
}

export default App;
