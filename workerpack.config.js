const path = require('path')
module.exports = {
    entry: './src/wasmworkers/workers.js',
    target: 'webworker',
    output: {
        path: path.join(__dirname, 'public'),
        filename: 'workers.js'
    }
}