/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    relative: true,
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    /* https://lospec.com/palette-list/graveyard-mist */
    colors: {
      gray: {
        300: "#b1b8a9",
        400: "#82857b",
        500: "#665f57",
        600: "#383730",
        700: "#0b0d0a",
      },
      blue: {
        400: "#95bdb2",
        500: "#5a7b85",
        600: "#273245",
      },
      green: {
        400: "#d4ea92",
        500: "#81ba78",
        600: "#477a5a",
      },
      beige: {
        400: "#dee6b8",
        500: "#b59766",
      },
      red: {
        400: "#8f5450",
        500: "#66403c",
        600: "#40292f",
      },
    },
    fontFamily: {
      serif: ["Yatra One", "serif"],
      sans: ["Rubik", "sans-serif"],
      monospace: ["PT Mono", "Courier", "monospace"],
    },
    extend: {},
  },
  plugins: [require("@tailwindcss/forms")],
};
