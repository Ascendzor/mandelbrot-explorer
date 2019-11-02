import React, {useEffect} from 'react'

const rustWorker = new Worker(process.env.NODE_ENV === 'development' ? '/workers.js' : '/mandelbrot-explorer/workers.js')
const jsWorker = new Worker(process.env.NODE_ENV === 'development' ? '/workers.js' : '/mandelbrot-explorer/workers.js')

// http://localhost:3000/#[-77.94416987757012,-304.7122994116724]%E2%82%BF40
const testCase = {
    // coords: {
    //     x: -1308729360660,
    //     y: 334767660538,
    //     z: 40
    // }
    // coords: {
    //     x: -1,
    //     y: 0,
    //     z: 1
    // }
    // -1292599, y: 332059, z: 20
    coords: {
        x: -1292599,
        y: 332059,
        z: 20
    }
}
export default () => {
    useEffect(() => {
        setTimeout(() => {
            (async () => {
                let rustData = null
                await new Promise((resolve, reject) => {
                    let counter = 0
                    rustWorker.onmessage = evt => {
                        counter++
                        if(counter === 100) {
                            console.timeEnd('rust')
                            resolve(evt.data.imageData)
                        }

                        // resolve(evt.data.imageData)
                    }
                    console.time('rust')
                    console.log('Starting rust benchmark')
                    console.log({...testCase})
                    console.log(`Rendering 100 times`)
                    Array.from({length: 100}).forEach(() => rustWorker.postMessage({
                        ...testCase,
                        computeOption: 'rust'
                    }))
                    // Array.from({length: 100}).forEach(() => rustWorker.postMessage(testCase))
                }).then(imageData => {
                    rustData = imageData
                })

                await new Promise((resolve, reject) => {
                    let counter = 0
                    jsWorker.onmessage = evt => {
                        counter++
                        if(counter === 100) {
                            console.timeEnd('js')
                            return resolve(evt.data.imageData)
                        }

                        resolve(evt.data.imageData)
                    }
                    console.time('js')
                    console.log('Starting js benchmark')
                    console.log({...testCase})
                    console.log(`Rendering 100 times`)
                    Array.from({length: 100}).forEach(() => jsWorker.postMessage({
                        ...testCase,
                        computeOption: 'js'
                    }))
                }).then(imageData => {
                    
                })
            })()
        }, 1000)
    }, [])

    return <div>
        benchmark
    </div>
}