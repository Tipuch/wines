const webpack = require('webpack');
const { VueLoaderPlugin } = require('vue-loader');
const VuetifyLoaderPlugin = require('vuetify-loader/lib/plugin');

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
                test: /\.s(c|a)ss$/,
                use: [
                    'vue-style-loader',
                    'css-loader',
                    {
                        loader: 'sass-loader',
                        // Requires sass-loader@^7.0.0
                        options: {
                            implementation: require('sass'),
                            fiber: require('fibers'),
                            indentedSyntax: true // optional
                        },
                        // Requires sass-loader@^8.0.0
                        options: {
                            implementation: require('sass'),
                            sassOptions: {
                                fiber: require('fibers'),
                                indentedSyntax: true // optional
                            }
                        }
                    }
                ]
            }
        ]
    },

    // File extensions to support resolving
    resolve: {
        extensions: ['.js']
    },

    plugins: [new VueLoaderPlugin(), new VuetifyLoaderPlugin()]
};
