const path = require('path');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const webpack = require('webpack');
const WorkerPlugin = require('worker-plugin');

module.exports = function override(config, env) {
  // config.resolve.extensions.push(".wasm");
  // config.module.rules.forEach(rule => {
  //   (rule.oneOf || []).forEach(oneOf => {
  //     if (oneOf.loader && oneOf.loader.indexOf("file-loader") >= 0) {
  //       // Make file-loader ignore WASM files
  //       oneOf.exclude.push(/\.wasm$/);
  //     }
  //   });
  // })
  return {...config,
    // module: { ...config.module,
    //   rules: [...config.module.rules,
    //     {
    //       test: /\.worker\.js$/,
    //       use: { loader: 'worker-loader' }
    //     }
        
    //   ]
    // },
    plugins: [...config.plugins, new WorkerPlugin()]
  }
}