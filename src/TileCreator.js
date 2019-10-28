const workerCount = 4
let workers = []
let workerPointer = 0
const taskWorker = async (iterations, r, i) => {
  console.log(workers)
  if(workers.length === 0) workers = await Promise.all(Array.from({length: workerCount}).map(a => window.getRustWorker()))
  workerPointer = (workerPointer+1) % workerCount
  return await workers[workerPointer].mandelbrot(iterations, r, i)
}

export default async ({coords, xBounds, yBounds, tileSize, maxIterations}) => {
  
  let iterations = []
  for(let y=0; y<tileSize; y++) for(let x=0; x<tileSize; x++) {
    const preNormalizedPixel = coords.x + (x/tileSize)
    const rangePercentile = ((preNormalizedPixel-xBounds.min) * 100) / (xBounds.max - xBounds.min)

    const ypreNormalizedPixel = coords.y + (y/tileSize)
    const yrangePercentile = ((ypreNormalizedPixel-yBounds.min) * 100) / (yBounds.max - yBounds.min)
    const real = (rangePercentile * (1 - -2) / 100) + -2
    const imaginary = (yrangePercentile * (1 - -1) / 100) + -1

    const iteration = await taskWorker(maxIterations*coords.z, real, imaginary)
    iterations.push(iteration)
  }

  return iterations
}