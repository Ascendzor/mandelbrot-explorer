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
