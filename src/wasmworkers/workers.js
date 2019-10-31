const rusty = import("./rust/pkg/mandelbrot.js")
const mandelbrotreference = require('./mandelbrotreference')

const colourScale = Array.from({length: 4096}).map((_, i) => {
    return {
        r: i % 256,
        g: (i+85) % 256,
        b: (i+(85*2)) % 256,
        a: 255
    }
})

let taskQueue = []
onmessage = message => new Promise((resolve, reject) => {
    taskQueue.push(message.data)
    work()
})

const work = async () => {
    const {coords, computeOption} = taskQueue.pop()

    rusty.then(rustyapi => {
        let imageData = null
        if(computeOption === 'js') {
            imageData = mandelbrotreference.default(coords.x, coords.y, coords.z)
            // iterations = mandelbrotreference.default(
            //     coords.x,
            //     coords.y,
            //     maxIterations,
            //     xBounds.max,
            //     xBounds.min,
            //     yBounds.max,
            //     yBounds.min
            // )
        } else {
            console.log({colourScale})
            
            imageData = rustyapi.mandelbrot(coords.x, coords.y, coords.z)
            imageData = new Uint8ClampedArray(imageData)
            imageData = new ImageData(imageData, 256, 256)
        }
        postMessage({
            coords,
            imageData
        })
    })

    if(taskQueue.length > 0) work()
}