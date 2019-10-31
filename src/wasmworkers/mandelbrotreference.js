import {tileSize, maxIterations} from '../constants'
import createColourScale from '../createColourScale'
const colourScale = createColourScale()

export default (xCoord, yCoord, zCoord) => {
    const imageData = new ImageData(tileSize, tileSize)
    yCoord = -yCoord
    zCoord = zCoord+1

    const minXBounds = -((2)**zCoord)
    return minXBounds
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
        const pixel  = (((tileSize-1-y) * tileSize) + x)
        
        const colour = colourScale[iteration]

        if(iteration === maxIterations*zCoord) {
            imageData.data[pixel*4+0] = 0
            imageData.data[pixel*4+1] = 0
            imageData.data[pixel*4+2] = 0
            imageData.data[pixel*4+3] = 255
            
        } else {
            imageData.data[pixel*4+0] = colour.r
            imageData.data[pixel*4+1] = colour.g
            imageData.data[pixel*4+2] = colour.b
            imageData.data[pixel*4+3] = colour.a
        }
    }

    return imageData
}