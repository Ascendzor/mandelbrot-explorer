const rusty = import("./rust/pkg/mandelbrot.js")
const mandelbrotreference = require('./mandelbrotreference')

let taskQueue = []
onmessage = message => new Promise((resolve, reject) => {
    taskQueue.push(message.data)
    work()
})

const work = async () => {
    const {coords, xBounds, yBounds, maxIterations, computeOption} = taskQueue.pop()
    rusty.then(rustyapi => {

        let iterations = null
        if(computeOption === 'js') {
            iterations = mandelbrotreference.default(coords, maxIterations, xBounds, yBounds)
        } else {
            iterations = rustyapi.mandelbrot(coords, maxIterations, xBounds, yBounds)
        }
        
        postMessage({
            coords,
            iterations
        })
    })

    if(taskQueue.length > 0) work()
}