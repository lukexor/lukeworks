/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "class",
  content: {
    relative: true,
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    fontFamily: {
      display: ["Yatra One", "serif"],
      body: ["Rubik", "sans-serif"],
      mono: ["PT Mono", "Courier", "monospace"],
    },
    extend: {
      height: {
        screen: ["100vh", "100dvh"],
      },
      minHeight: {
        screen: ["100vh", "100dvh"],
      },
      backgroundImage: {
        "intro-texture": "url('/images/code-bg.webp')",
      },
    },
  },
  plugins: [require("@tailwindcss/forms")],
};
