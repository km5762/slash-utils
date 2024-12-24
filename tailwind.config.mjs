/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}"],
  theme: {
    extend: {},
  },
  plugins: [
    function ({ addUtilities }) {
      const newUtilities = {
        ".hover-overlay": {
          position: "relative",
        },
        ".hover-overlay::before": {
          content: "''",
          position: "absolute",
          top: "0",
          left: "0",
          width: "100%",
          height: "100%",
          backgroundColor: "rgba(255, 255, 255, 0)",
          transition: "background-color 0.3s ease-in-out",
          borderRadius: "inherit",
        },
        ".hover-overlay:hover::before": {
          backgroundColor: "rgba(255, 255, 255, var(--overlay-opacity, 0.15))",
        },
      };

      addUtilities(newUtilities, ["hover"]);
    },
  ],
};
