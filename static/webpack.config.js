const path = require('path');
const webpack = require('webpack');

module.exports = {
    // Set debugging source maps to be "inline" for
    // simplicity and ease of use
    mode: 'production',
    devtool: 'inline-source-map',
    // The application entry point
    entry: './js/index.ts',
    // Where to compile the bundle
    // By default the output directory is `dist`
    output: {
        filename: 'bundle.js'
    },

    // Supported file loaders
    module: {
        rules: [
            {
                test: /\.ts?$/,
                loader: 'ts-loader'
            },
            {
                test: /\.css$/i,
                use: ['style-loader', 'css-loader']
            }
        ]
    },

    // File extensions to support resolving
    resolve: {
        extensions: ['.ts', '.js']
    }
};
