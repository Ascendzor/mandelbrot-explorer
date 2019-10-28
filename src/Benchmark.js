import React, {useEffect} from 'react'
import PubSub from 'pubsub-js'

const rustWorker = new Worker('/workers.js')
const jsWorker = new Worker('/workers.js')

// http://localhost:3000/#[-77.94416987757012,-304.7122994116724]%E2%82%BF40
const testCase = {
    coords: {
        x: -1308729360660,
        y: 334767660538,
        z: 41
    },
    maxIterations: 50,
    tileSize: 256,
    xBounds: {
        min: -2199023255552,
        max: 1099511627776
    },
    yBounds: {
        min: -1099511627776,
        max: 1099511627776
    }
}
export default () => {
    useEffect(() => {
        console.time('rust')
        new Promise((resolve, reject) => {
            rustWorker.onmessage = evt => {
                console.timeEnd('rust')
                resolve()
            }
        }).then(() => new Promise((resolve, reject) => {
            console.time('js')
            jsWorker.onmessage = e => {
                console.timeEnd('js')
                resolve()
            }
            jsWorker.postMessage({...testCase, computeOption: 'js'})
        }))
        
        rustWorker.postMessage(testCase)
    }, [])

    return <div>
        benchmark
    </div>
}