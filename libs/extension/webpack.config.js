const path = require('path')
const CopyPlugin = require('copy-webpack-plugin')
module.exports = [

  {
    mode: 'production',
    entry: {
      popup: path.resolve(__dirname, 'extension_js', 'js', 'popup.ts'),
      options: path.resolve(__dirname, 'extension_js', 'js', 'options.ts'),
      content: path.resolve(__dirname, 'extension_js', 'js', 'content.ts')
    },
    output: {
      path: path.resolve(__dirname, 'extension_js', 'js'),
      filename: '[name].js'
    },
    resolve: {
      extensions: ['.ts', '.js']
    },
    module: {
      rules: [
        {
          test: /\.tsx?$/,
          loader: 'ts-loader',
          exclude: /node_modules/
        }
      ]
    },
    target: 'webworker',
    plugins: [
      new CopyPlugin({
        patterns: [
          { from: '.', to: '.', context: 'public' },
          { from: './extension_js/manifests/manifest_cr.json', to: 'manifest.json' }
        ]
      })
    ]
  },
  {
    mode: 'production',
    entry: {
      popup: path.resolve(__dirname, 'extension_js', 'js', 'popup.ts'),
      options: path.resolve(__dirname, 'extension_js', 'js', 'options.ts'),
      content: path.resolve(__dirname, 'extension_js', 'js', 'content.ts')
    },
    output: {
      path: path.resolve(__dirname, 'extension_js', 'js'),
      filename: '[name].js'
    },
    resolve: {
      extensions: ['.ts', '.js']
    },
    module: {
      rules: [
        {
          test: /\.tsx?$/,
          loader: 'ts-loader',
          exclude: /node_modules/
        }
      ]
    },
    target: 'web',
    plugins: [
      new CopyPlugin({
        patterns: [
          { from: '.', to: '.', context: 'public' },
          { from: './extension_js/manifests/manifest_cr.json', to: 'manifest.json' }
        ]
      })
    ]
  }
]