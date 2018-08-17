const maxIterations = 50
const width = 256
const height = 256
const normalize = (val, max, min) => (val-min) / (max-min)
const theMandelbrot = (z, c) => {
  return {
    x: z.x*z.x - z.y*z.y + c.x,
    y: 2 * z.x * z.y + c.y
  }
}

export default (imgData, width, height) => {
  let exes = []

  for(let x=0; x<width; x++) for(let y=0; y<height; y++) {
    if(!exes[x]) exes[x] = []
    const normalizedX = normalize(x, width, 0)
    const normalizedY = normalize(y, height, 0)
    const real = (normalizedX*3.5) - 2.5      //These magic numbers are for the mandelbrot scale
    const imaginary = (normalizedY*2) - 1     //These magic numbers are for the mandelbrot scale

    // console.log(real, imaginary)

    let iteration = 0
    let z = {x: real, y: imaginary}
    const c = {x: real, y: imaginary}
    while((z.x**2+z.y**2)<2**2 && iteration<maxIterations) {
      z = theMandelbrot(z, c)
      iteration++
    }
    if(iteration === maxIterations) {
      imgData.data[(x*width+y)*4+0] = 255
      imgData.data[(x*width+y)*4+1] = 0
      imgData.data[(x*width+y)*4+2] = 0
      imgData.data[(x*width+y)*4+3] = 255
      exes[x][y] = '-'
    } else {
      exes[x][y] = 'x'
      imgData.data[(x*width+y)*4+0] = 0
      imgData.data[(x*width+y)*4+1] = 0
      imgData.data[(x*width+y)*4+2] = 0
      imgData.data[(x*width+y)*4+3] = 255
    }
  }
  // exes.forEach((ys, x) => {
  //   console.log(ys.join(' '))
  // })
  return imgData
}
