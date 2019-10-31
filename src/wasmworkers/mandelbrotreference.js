import {tileSize, maxIterations} from '../constants'

export default (xCoord, yCoord, zCoord) => {
    const imageData = new ImageData(tileSize, tileSize)
    yCoord = -yCoord
    zCoord = zCoord+1

    const minXBounds = -((2)**zCoord)
    const maxXBounds = -minXBounds/2

    const minYBounds = minXBounds/2
    const maxYBounds = -minYBounds
    // console.log('js: ' + [minXBounds, maxXBounds, minYBounds, maxYBounds].join(' '))

    for(let y=0; y<tileSize; y++) for(let x=0; x<tileSize; x++) {
        const preNormalizedPixel = xCoord + (x/tileSize)
        const rangePercentile = ((preNormalizedPixel-minXBounds) * 100) / (maxXBounds - minXBounds)

        const ypreNormalizedPixel = yCoord + (y/tileSize)
        const yrangePercentile = ((ypreNormalizedPixel-minYBounds) * 100) / (maxYBounds - minYBounds)
        const imaginary = (yrangePercentile * (1 - -1) / 100) + -1
        const real = (rangePercentile * (1 - -2) / 100) + -2
        const pixel  = (((tileSize-1-y) * tileSize) + x)

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
        

        imageData.data[pixel*4+0] = iteration / 4
        imageData.data[pixel*4+1] = iteration / 2
        imageData.data[pixel*4+2] = iteration
        imageData.data[pixel*4+3] = 255
    }

    return imageData
}