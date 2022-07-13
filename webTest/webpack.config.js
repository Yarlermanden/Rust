const webpack = require('webpack')
const path = require('path');

function test12() {
  console.log("webpack");
}

module.exports = {
    mode: 'development',
    entry: path.resolve(__dirname, './index.js'),
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'bundle.js',
    },
}
