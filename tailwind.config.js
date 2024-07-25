/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./libs/**/*.{html,js,rs}', '!./libs/**/node_modules'],
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
        'off-white': '#EDEDED'
      },
      fontFamily: {
        'bebas-neue': ['"Bebas Neue"', 'sans-serif'],
        'open-sans': ['"Open Sans"', 'sans-serif']
      },
      fontWeight: {
        regular: 400,
        semibold: 600 // Define semibold weight
      }
    }
  },
  plugins: []
}