import {tileSize, maxIterations} from '../constants'

export default (xCoord, yCoord, zCoord) => {
    const imageData = []

    const minXBounds = -((2)**zCoord)
    const maxXBounds = -minXBounds/2

    const minYBounds = minXBounds/2
    const maxYBounds = -minYBounds

    for(let y=0; y<tileSize; y++) for(let x=0; x<tileSize; x++) {
        const preNormalizedPixel = xCoord + (x/tileSize)
        const rangePercentile = ((preNormalizedPixel-minXBounds) * 100) / (maxXBounds - minXBounds)

        const ypreNormalizedPixel = yCoord + (y/tileSize)
        const yrangePercentile = ((ypreNormalizedPixel-minYBounds) * 100) / (maxYBounds - minYBounds)
        const imaginary = (yrangePercentile * (1 - -1) / 100) + -1
        const real = (rangePercentile * (1 - -2) / 100) + -2

        let iteration = 0
        let z = {x: real, y: imaginary}
        const c = {x: real, y: imaginary}
        while((z.x**2+z.y**2)<2**2 && iteration<maxIterations*zCoord) {
            z = {
                x: z.x*z.x - z.y*z.y + c.x,
                y: 2 * z.x * z.y + c.y
            }
            iteration++
        }
        

        imageData.push(iteration / 4)
        imageData.push(iteration / 2)
        imageData.push(iteration)
        imageData.push(255)
    }

    return imageData
}