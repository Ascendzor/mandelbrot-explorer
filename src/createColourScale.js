import {times} from 'lodash'

const range = 4096

const colourScale = times(range, i => {
  return {
    r: i % 256,
    g: (i+85) % 256,
    b: (i+(85*2)) % 256,
    a: 255
  }
})

export default () => {
  return colourScale
}

export const redColourScale = () => {
  return times(range, i => {
    return i % 256
  })
}
export const greenColourScale = () => {
  return times(range, i => {
    return (i+85) % 256
  })
}
export const blueColourScale = () => {
  return times(range, i => {
    return (i+(85*2)) % 256
  })
}
