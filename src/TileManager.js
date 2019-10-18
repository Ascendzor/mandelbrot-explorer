import TileCreator from './TileCreator'
import PubSub from 'pubsub-js'
import {isEqual} from 'lodash'
import {spawn, Pool} from 'threads'
import wasmWorker from 'wasm-worker'
import {instantiateStreaming} from "assemblyscript/lib/loader"

// fetch('mandelbrot_bg.wasm', {
//   headers: {
//     'Content-Type': 'application/wasm'
//   }
// }).then(response => {
//   console.log('something')
//   return response.arrayBuffer()
// }).then(bytes => {
//   console.log('2')
//   console.log({bytes})
//   return 
// }).then(obj => {
//     console.log('made it')
//     console.log(obj)
// })
const theFetch = () => {
  return fetch('mandelbrot_bg.wasm', {
    headers: {
      'Content-Type': 'application/wasm'
    }
  }).then(console.log)
}
let importObject = { imports: { imported_func: arg => console.log(arg) } };
window.WebAssembly.instantiateStreaming(theFetch, importObject).then(something => {
  console.log({something})
})

// supposing an "add.wasm" module that exports a single function "add"
// wasmWorker('/mandelbrot_bg.wasm').then(mandy => {
//   console.log(mandy)
//   return mandy.exports.add(1, 2);
// }).then(sum => {
//   console.log('1 + 2 = ' + sum);
// }).catch(ex => {
//   // ex is a string that represents the exception
//   console.error(ex);
// })

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

  if(coords.z !== zoom) {
    while(jobs.length > 1) jobs.pop().abort()
    zoom = coords.z
  }


  jobs.push(pool.run(TileCreator).send({coords, xBounds, yBounds, tileSize, maxIterations}))
  pool.on('done', (job, response) => {
    if(isEqual(response.coords, coords)) {
      tiles[getTileKey(response.coords)] = response.iterations
      resolve(response.iterations)
    }
  })
})
