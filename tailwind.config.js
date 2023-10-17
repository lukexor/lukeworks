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
        100: "#f7f7f7",
        200: "#e5e9e1",
        300: "#d4d8cd",
        400: "#82857b",
        500: "#665f57",
        600: "#383730",
        700: "#1b1d1a",
        800: "#0b0d0a",
      },
      blue: {
        300: "#95bdb2",
        400: "#7b9a9a",
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
        600: "#7e5f3d",
      },
      red: {
        300: "#e7b7b0",
        400: "#8f5450",
        500: "#66403c",
        600: "#40292f",
      },
    },
    fontFamily: {
      display: ["Yatra One", "serif"],
      body: ["Rubik", "sans-serif"],
      mono: ["PT Mono", "Courier", "monospace"],
    },
    extend: {
      height: {
        screen: ["100vh", "100dvh"],
      },
      backgroundImage: {
        "intro-texture": "url('/images/code-bg.webp')",
      },
    },
  },
  plugins: [require("@tailwindcss/forms")],
};
