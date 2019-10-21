import TileCreator from './TileCreator'
import PubSub from 'pubsub-js'
import {isEqual} from 'lodash'
import {Pool} from 'threads'
import wasmWorker from 'wasm-worker'
import {instantiateStreaming} from "assemblyscript/lib/loader"

const pool = new Pool(Math.min(2, navigator.hardwareConcurrency - 2))

let tiles = []
let jobs = []

const getTileKey = coords => {
  return Object.values(coords).join(' ')
}

let zoom = null

export const getIterationsForTile = ({coords, xBounds, yBounds, tileSize, maxIterations}) => new Promise((resolve, reject) => {
  const tile = tiles[getTileKey(coords)]
  if(tile) return resolve(tile)
  
  // if(coords.z !== zoom) {
  //   while(jobs.length > 1) jobs.pop().abort()
  //   zoom = coords.z
  // }

  jobs.push(pool.run(TileCreator).send({coords, xBounds, yBounds, tileSize, maxIterations}))
  pool.on('done', (job, response) => {
    if(isEqual(response.coords, coords)) {
      tiles[getTileKey(response.coords)] = response.iterations
      resolve(response.iterations)
    }
  })
  // return TileCreator({coords, xBounds, yBounds, tileSize, maxIterations}).then(response => {
  //   tiles[getTileKey(coords)] = response.iterations
  //   return resolve(response.iterations)
  // })
  
  
})
