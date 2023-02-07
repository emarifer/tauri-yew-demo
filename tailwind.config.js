/** @type {import('tailwindcss').Config} */
const defaultTheme = require("tailwindcss/defaultTheme");

module.exports = {
  content: [
    "./src/**/*.rs",
    "./index.html",
    "./src/**/*.html",
    "./src/**/*.css",
  ],
  theme: {
    extend: {
      colors: {
        primary: "#282c34",
      },
      fontFamily: {
        comic: ["Comic", ...defaultTheme.fontFamily.sans],
        kanit: ["Kanit", ...defaultTheme.fontFamily.sans],
        stint: ["Stint", ...defaultTheme.fontFamily.sans],
        ubuntu: ["Ubuntu", ...defaultTheme.fontFamily.sans],
      },
    },
  },
  plugins: [],
};

/*
 * FUENTES ONLINE PARA YEW. VER:
 * https://fonts.google.com/
 * https://www.onlinewebfonts.com/top
 */

/*
 * COPIAR UN ASSETS A LA CARPETA DIST:
 * https://trunkrs.dev/assets/#copy-file
 */
