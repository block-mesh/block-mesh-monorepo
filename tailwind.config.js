/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./libs/**/*.{html,js,rs}"],
    theme: {
        extend: {
            fontFamily: {
                sans: ['Inter, sans-serif', {fontFeatureSettings: '"cv11"'}],
                jetbrains: ['JetBrains Mono', 'monospace'],
            },
            colors: {
                blue: '#25e4d3 !important',
                darkBlue: '#2f8985 !important',
                orange: '#fab457 !important',
                darkOrange: '#f97432 !important',
                light: '#5558 !important',
                lightDark: '#8881 !important',
                dark: '#202525 !important',
            },
            boxShadow: {
                'dark': '-5px 5px 10px 5px #111 !important',
                'blue': '0 0 3px #25e4d3 !important',
                'light': '0 0 3px #fff8 !important',
                'darkOrange': '0 0 3px #f97432 !important',
            },
            fontSize: {
                'large': '2.3em',
                'medium': '1.5em',
            },
            backgroundImage: theme => ({
                'bandwidth-card': "url('https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/4c23603d-d4af-40bb-19b2-91db0b51ff00/public')",
            }),
            rotate: {
                '180': '180deg',
            },
        },
    },
    plugins: [],
}