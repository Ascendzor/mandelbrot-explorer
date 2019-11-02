const PubSub = require('pubsub-js');

const getTileKey = coords => {
  return [coords.x, coords.y, coords.z].join(' ')
}

const theWorkers = Array.from({length: 4}).map(a => {
  if(process.env.NODE_ENV === 'development') return new Worker('/workers.js')

  //github pages, https://ascendzor.github.io/mandelbrot-explorer/
  return new Worker('/mandelbrot-explorer/workers.js')
})
theWorkers.forEach(worker => {
  worker.onmessage = evt => {
    PubSub.publish('onTileLoad', evt.data)
  }
})

let tiles = {}
let workerPointer = 0

export const renderTile = ({coords, computeOption}) => new Promise((resolve, reject) => {
  PubSub.subscribe('onTileLoad', (eventName, loadedTile) => {
    const tileKey = getTileKey(coords)
    const loadedTileKey = getTileKey(loadedTile.coords)
    if(tileKey === loadedTileKey) {
      return resolve(loadedTile.imageData)
    }
  })

  theWorkers[workerPointer].postMessage({coords, computeOption})
  workerPointer = (workerPointer+1) % theWorkers.length
})