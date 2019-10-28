const rusty = import("./rust/pkg/mandelbrot.js")
const mandelbrotreference = require('./mandelbrotreference')

let taskQueue = []
onmessage = message => new Promise((resolve, reject) => {
    taskQueue.push(message.data)
    work()
})

const work = async () => {
    const {coords, xBounds, yBounds, tileSize, maxIterations, computeOption} = taskQueue.pop()
    rusty.then(rustyapi => {
        let iterations = []
        for(let y=0; y<tileSize; y++) for(let x=0; x<tileSize; x++) {
            const preNormalizedPixel = coords.x + (x/tileSize)
            const rangePercentile = ((preNormalizedPixel-xBounds.min) * 100) / (xBounds.max - xBounds.min)

            const ypreNormalizedPixel = coords.y + (y/tileSize)
            const yrangePercentile = ((ypreNormalizedPixel-yBounds.min) * 100) / (yBounds.max - yBounds.min)
            const real = (rangePercentile * (1 - -2) / 100) + -2
            const imaginary = (yrangePercentile * (1 - -1) / 100) + -1

            let iteration = 0
            
            if(computeOption === 'js') iteration = mandelbrotreference.default(maxIterations*coords.z, real, imaginary)
            else iteration = rustyapi.mandelbrot(maxIterations*coords.z, real, imaginary)

            iterations.push(iteration)
        }
        postMessage({
            coords,
            iterations
        })
    })

    if(taskQueue.length > 0) work()
}