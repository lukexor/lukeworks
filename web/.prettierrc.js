/** @type {import('@types/prettier').Config} */
module.exports = {
  arrowParens: "always",
  bracketSpacing: true,
  printWidth: 80,
  quoteProps: "as-needed",
  semi: true,
  singleQuote: false,
  jsxSingleQuote: false,
  bracketSameLine: false,
  tabWidth: 2,
  trailingComma: "all",
  useTabs: false,
  plugins: ["prettier-plugin-organize-imports"],
};
