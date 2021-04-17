import postcss from 'rollup-plugin-postcss'
import path from "path"

export default {
  plugins: [
    postcss({
      extract: path.resolve("./pkg/bundle.css"),
      plugins: [
        require('postcss-import'),
        require('tailwindcss')("tailwind.config.js"),
        require('autoprefixer'),
      ]
    })
  ]
}