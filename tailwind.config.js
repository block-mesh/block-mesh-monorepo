/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    '!./node_modules',
    '!./libs/react-native-app/node_modules',
    './libs/**/*.{html,js,rs}'
  ],
  /*
  safelist: [
      {
          pattern: /./
      }
  ],
   */
  theme: {
    extend: {
      colors: {
        'dark-blue': '#0C1120',
        'magenta': '#DA00DB',
        'magenta-1': '#FE49FF',
        'magenta-2': '#E426E6',
        'magenta-3': '#A700A8',
        'magenta-4': '#670068',
        'cyan': '#01FEFC',
        'orange': '#FF7E07',
        'off-white': '#EDEDED',
        // 'blue': '#25e4d3',
        // 'darkBlue': '#0C1120',
        // 'light': '#EDEDED',
        // 'darkOrange': '#f97432',
        // 'dark': '#0C1120',
        'light-blue': '#25e4d3',
        darkBlue: '#2f8985',
        // orange: '#fab457',
        darkOrange: '#f97432',
        light: '#55555588',
        lightDark: '#88888811',
        dark: '#202525'
      },
      boxShadow: {
        'dark': '-5px 5px 10px 5px #111111 !important',
        'blue': '0 0 3px #25e4d3 !important',
        'light': '0 0 3px #ffffff8 !important',
        'darkOrange': '0 0 3px #f97432 !important'
        // 'dark': '-5px 5px 10px 5px #0C1120',
        // 'blue': '0 0 3px #25e4d3',
        // 'light': '0 0 3px #EDEDED',
        // 'darkOrange': '0 0 3px #FF7E07',
      },
      fontFamily: {
        sans: ['Inter, sans-serif', { fontFeatureSettings: '"cv11"' }],
        jetbrains: ['JetBrains Mono', 'monospace'],
        'bebas-neue': ['"Bebas Neue"', 'sans-serif'],
        'open-sans': ['"Open Sans"', 'sans-serif']
      },
      fontWeight: {
        regular: 400,
        semibold: 600 // Define semibold weight
      },
      backgroundImage: theme => ({
        'bandwidth-card': 'url(\'https://r2-images.blockmesh.xyz/4c23603d-d4af-40bb-19b2-91db0b51ff00.png\')'
      }),
      rotate: {
        '180': '180deg'
      }
    }
  },
  plugins: []
}
