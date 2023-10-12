/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "class",
  content: {
    relative: true,
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    /* https://lospec.com/palette-list/graveyard-mist */
    colors: {
      transparent: "transparent",
      current: "currentColor",
      white: "#ffffff",
      black: "#000000",
      gray: {
        300: "#b1b8a9",
        400: "#82857b",
        500: "#665f57",
        600: "#383730",
        700: "#1b1d1a",
        800: "#0b0d0a",
      },
      blue: {
        400: "#95bdb2",
        500: "#5a7b85",
        600: "#273245",
        700: "#131821",
        800: "#0e1116",
      },
      green: {
        400: "#d4ea92",
        500: "#81ba78",
        600: "#477a5a",
      },
      yellow: {
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
    extend: {
      height: {
        screen: ["100vh", "100dvh"],
      },
    },
  },
  plugins: [require("@tailwindcss/forms")],
};
