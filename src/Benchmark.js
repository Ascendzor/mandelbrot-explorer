import React, {useEffect} from 'react'

const rustWorker = new Worker('/workers.js')
const jsWorker = new Worker('/workers.js')

// http://localhost:3000/#[-77.94416987757012,-304.7122994116724]%E2%82%BF40
const testCase = {
    coords: {
        x: -1308729360660,
        y: 334767660538,
        z: 41
    }
}
export default () => {
    useEffect(() => {
        setTimeout(() => {
            (async () => {
                await new Promise((resolve, reject) => {
                    let counter = 0
                    rustWorker.onmessage = evt => {
                        console.log(evt.data)
                        counter++
                        if(counter === 100) {
                            console.timeEnd('rust')
                            resolve()
                        }
                    }
                    console.time('rust')
                    Array.from({length: 1}).forEach(() => rustWorker.postMessage(testCase))
                })

                await new Promise((resolve, reject) => {
                    let counter = 0
                    jsWorker.onmessage = evt => {
                        counter++
                        if(counter === 100) console.timeEnd('js')
                    }
                    console.time('js')
                    Array.from({length: 1}).forEach(() => jsWorker.postMessage(testCase))
                })
            })()
        }, 1000)
    }, [])

    return <div>
        benchmark
    </div>
}