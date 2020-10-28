const CopyWebpackPlugin = require("copy-webpack-plugin");
const VueLoaderPlugin = require('vue-loader/lib/plugin');

const path = require('path');

module.exports = {
  entry: {
    vue: "./src/main.js"
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "[name].js",
  },
  mode: "development",
  module: {
    rules: [
      {
        test: /\.vue$/,
        loader: 'vue-loader'
      },
      // this will apply to both plain `.js` files
      // AND `<script>` blocks in `.vue` files
      {
        test: /\.js$/,
        loader: 'babel-loader'
      },
    ]
  },
  plugins: [
    new VueLoaderPlugin(),
    new CopyWebpackPlugin(['index.html']),
  ],
};
