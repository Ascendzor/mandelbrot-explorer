import React from 'react';
import './App.css';
import Mandelbrot from './Mandelbrot'
import Benchmark from './Benchmark'

export default () => {
  if(window.location.pathname === '/benchmark') {
    return <div className="App">
      <div>
        Do the benchmark here
      </div>
      <Benchmark />
    </div>
  }

  return <div className="App">
    <Mandelbrot />
  </div>
};
