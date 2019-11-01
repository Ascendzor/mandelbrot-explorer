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
            imageData = mandelbrotreference.default(coords.x, coords.y, coords.z+1)
            imageData = new Uint8ClampedArray(imageData)
            imageData = new ImageData(imageData, 256, 256)
        } else {
            imageData = rustyapi.mandelbrot(coords.x, coords.y, coords.z+1)
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