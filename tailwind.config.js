/* @type {import('tailwindcss').Config} */
/* Tailwind 4.0 doesn't need a config but tailwindcss-intellisense still
 * requires it to work */
module.exports = {
  content: {
    relative: true,
    files: ["*.html", "./src/**/*.rs"],
  },
};
