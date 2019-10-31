import React, {useEffect} from 'react'

const rustWorker = new Worker('/workers.js')
const jsWorker = new Worker('/workers.js')

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
                        if(counter === 100) console.timeEnd('rust')

                        resolve(evt.data.imageData)
                    }
                    console.time('rust')
                    Array.from({length: 1}).forEach(() => rustWorker.postMessage(testCase))
                }).then(imageData => {
                    console.log('-------rust---------')
                    console.log(imageData)
                    rustData = imageData
                })

                await new Promise((resolve, reject) => {
                    let counter = 0
                    jsWorker.onmessage = evt => {
                        counter++
                        if(counter === 100) console.timeEnd('js')

                        resolve(evt.data.imageData)
                    }
                    console.time('js')
                    Array.from({length: 1}).forEach(() => jsWorker.postMessage({
                        ...testCase,
                        computeOption: 'js'
                    }))
                }).then(imageData => {
                    console.log('-------js---------')
                    console.log(imageData)
                    if(imageData == rustData) console.log('they matched')
                })
            })()
        }, 1000)
    }, [])

    return <div>
        benchmark
    </div>
}