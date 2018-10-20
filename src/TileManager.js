import TileCreatorWorker from './TileCreator.worker'
import TileCreator from './TileCreator'
import PubSub from 'pubsub-js'
import {isEqual} from 'lodash'
import {spawn, Pool} from 'threads'

const pool = new Pool()
let tiles = []

const getTileKey = coords => {
  return Object.values(coords).join(' ')
}
export const getIterationsForTile = ({coords, xBounds, yBounds, tileSize, maxIterations}) => new Promise((resolve, reject) => {
  const tile = tiles[getTileKey(coords)]
  if(tile) return resolve(tile)

  pool.run(TileCreator).send({coords, xBounds, yBounds, tileSize, maxIterations})
  pool.on('done', (job, response) => {
    if(isEqual(response.coords, coords)) {
      tiles[getTileKey(response.coords)] = response.iterations
      resolve(response.iterations)
    }
  })
})
