/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './index.html',
    './src/**/*.{js,jsx,ts,tsx}',
    flowbite.content(),
  ],
  theme: {
    extend: {
      keyframes: {
        borderPulse: {
          '0%, 100%': { borderColor: 'transparent' },
          '50%': { borderColor: 'rgb(59, 130, 246)' }, // Tailwind's blue-500
        },
      },
      animation: {
        borderPulse: 'borderPulse 2s infinite',
      },
    }
  },
  plugins: [
    flowbite.plugin(),
  ],
  variants: {
    extend: {
      scrollbar: ['hidden'],
    },
  },
}

