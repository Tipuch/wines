const webpack = require('webpack');
const { VueLoaderPlugin } = require('vue-loader');

module.exports = {
    // Set debugging source maps to be "inline" for
    // simplicity and ease of use
    mode: 'development',
    devtool: 'inline-source-map',
    // The application entry point
    entry: './js/index.js',
    // Where to compile the bundle
    // By default the output directory is `dist`
    output: {
        filename: 'bundle.js'
    },

    // Supported file loaders
    module: {
        rules: [
            {
                test: /\.js$/,
                exclude: /node_modules/,
                use: 'babel-loader'
            },
            {
                test: /\.vue$/,
                loader: 'vue-loader'
            },
            {
                test: /\.css$/i,
                use: ['style-loader', 'css-loader']
            }
        ]
    },

    // File extensions to support resolving
    resolve: {
        extensions: ['.js']
    },

    plugins: [new VueLoaderPlugin()]
};
