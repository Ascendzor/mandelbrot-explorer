const PubSub = require('pubsub-js');

const getTileKey = coords => {
  return [coords.x, coords.y, coords.z].join(' ')
}

const theWorkers = Array.from({length: 2}).map(a => new Worker('/workers.js'))
theWorkers.forEach(worker => {
  worker.onmessage = evt => {
    console.log('worker on message')
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

export const getIterationsForTile = ({tileCoords, xBounds, yBounds, tileSize, maxIterations}) => new Promise((resolve, reject) => {
  const tileKey = getTileKey(tileCoords)
  const tile = tiles[tileKey]
  if(tile) return resolve(tile)
  PubSub.subscribe('onTileLoad', (eventName, loadedTile) => {
    const loadedTileKey = getTileKey(loadedTile.coords)
    if(tileKey === loadedTileKey) {
      tiles[loadedTileKey] = loadedTile.iterations
      return resolve(loadedTile.iterations)
    }
  })

  console.log({coords: tileCoords, xBounds, yBounds, tileSize, maxIterations})
  theWorkers[workerPointer].postMessage({coords: tileCoords, xBounds, yBounds, tileSize, maxIterations})
  workerPointer = (workerPointer+1) % theWorkers.length
})
