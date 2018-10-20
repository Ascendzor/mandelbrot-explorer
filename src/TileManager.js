import TileCreator from './TileCreator.worker'
import PubSub from 'pubsub-js'
import {isEqual} from 'lodash'


let tiles = []

const getTileKey = coords => {
  return Object.values(coords).join(' ')
}
export const getIterationsForTile = ({coords, xBounds, yBounds, tileSize, maxIterations}) => new Promise((resolve, reject) => {
  const tile = tiles[getTileKey(coords)]
  if(tile) return resolve(tile)

  window.tileCreator = new TileCreator()
  window.tileCreator.postMessage({coords, xBounds, yBounds, tileSize, maxIterations})
  window.tileCreator.addEventListener('message', event => {
    const {coords, iterations} = event.data
    PubSub.publish('tileLoaded', event.data)
  })
  PubSub.subscribe('tileLoaded', (msg, data) => {
    if(isEqual(data.coords, coords)) {
      tiles[getTileKey(data.coords)] = data.iterations
      resolve(data.iterations)
    }
  })
})
