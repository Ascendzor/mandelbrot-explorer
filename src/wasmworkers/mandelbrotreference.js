import {tileSize} from '../constants'

export default (coords, max_iter, xBounds, yBounds) => {
    let iterations = []

    Array.from({length: tileSize}).forEach(y => Array.from({length: tileSize}).forEach(x => {
        const preNormalizedPixel = coords.x + (x/tileSize)
        const rangePercentile = ((preNormalizedPixel-xBounds.min) * 100) / (xBounds.max - xBounds.min)

        const ypreNormalizedPixel = coords.y + (y/tileSize)
        const yrangePercentile = ((ypreNormalizedPixel-yBounds.min) * 100) / (yBounds.max - yBounds.min)
        const real = (rangePercentile * (1 - -2) / 100) + -2
        const imaginary = (yrangePercentile * (1 - -1) / 100) + -1

        let zrzi = (real, imaginary)
        let iter = 0
        
        while (((zrzi[0]*zrzi[0]) + (zrzi[1]*zrzi[1]) <= 4.0) && (iter < max_iter)) {
            zrzi = (
                ((zrzi[0]*zrzi[0]) - (zrzi[1]*zrzi[1]) + real),
                ((2.0 * zrzi[0] * zrzi[1]) + imaginary)
            )
            iter = iter + 1
        }
        iterations.push(iter)
    }))

    return iterations
}