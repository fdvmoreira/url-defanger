/** @type {import('tailwindcss').Config} */
module.exports = {
  purge: {
    mode: "all",
    content: [
      "./index.html",
      "./src/**/*.html",
      "./src/**/*.rs",
      "./src/**/*.css",
    ],
  },
  content: ["./*.{html,js,css,rs}", "./src/**/*.{rs,html,css,js}"],
  theme: {
    extend: {},
  },
  plugins: [],
};
