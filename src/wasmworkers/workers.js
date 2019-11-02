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
            console.log(`Rendering {${[coords.x, coords.y, coords.z].join(' ')}} tile with js`)
            imageData = mandelbrotreference.default(coords.x, coords.y, coords.z+1)
            imageData = new Uint8ClampedArray(imageData)
            imageData = new ImageData(imageData, 256, 256)
        } else if (computeOption === 'rust') {
            console.log(`Rendering {${[coords.x, coords.y, coords.z].join(' ')}} tile with rust`)
            imageData = rustyapi.mandelbrot(coords.x, coords.y, coords.z+1)
            imageData = new Uint8ClampedArray(imageData)
            imageData = new ImageData(imageData, 256, 256)
        } else {
            console.log('Please pass a compute option such as "js" or "rust"')
        }
        postMessage({
            coords,
            imageData
        })
    })

    if(taskQueue.length > 0) work()
}