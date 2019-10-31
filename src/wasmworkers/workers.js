const rusty = import("./rust/pkg/mandelbrot.js")
const mandelbrotreference = require('./mandelbrotreference')

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
            imageData = rustyapi.mandelbrot(coords.x, coords.y, coords.z)
        }
        postMessage({
            coords,
            imageData
        })
    })

    if(taskQueue.length > 0) work()
}