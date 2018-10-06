import TileCreator from './TileCreator.worker'
import PubSub from 'pubsub-js'
import {isEqual} from 'lodash'

window.tileCreator = new TileCreator()
window.tileCreator.addEventListener('message', event => {
  const {coords, iterations} = event.data
  PubSub.publish('tileLoaded', event.data)
})

export const getIterationsForTile = ({coords, xBounds, yBounds, tileSize, maxIterations}) => new Promise((resolve, reject) => {
  window.tileCreator.postMessage({coords, xBounds, yBounds, tileSize, maxIterations})
  PubSub.subscribe('tileLoaded', (msg, data) => {
    if(isEqual(data.coords, coords)) {
      resolve(data.iterations)
    }
  })
})
