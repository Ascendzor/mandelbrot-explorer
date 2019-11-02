import React from 'react';
import './App.css';
import TheMap from './TheMap'
import Benchmark from './Benchmark'

export default () => {
  const hashPath = window.location.hash.split('#')[1]
  if(hashPath === 'benchmark') {
    return <div className="App">
      <Benchmark />
    </div>
  }

  return <div className="App">
    <TheMap />
  </div>
};
