import tailwindcssform from "@tailwindcss/forms";
export default {
    content: [
        "./devhtml/**/*.{html,js,ts,jsx,tsx}",
        "./src/**/*.rs"
    ],
    theme: {
      extend: {
        backgroundImage: {
          button: "linear-gradient(97.93deg, #ffeaa1 0%, #ffcc15 63.99%)",
        },
        colors: {
          /* Primary */
          "primary-main": "#FFCC15",
          "primary-surface": "#FFFAE7",
          "primary-border": "#FFEEA1",
          "primary-hover": "#BB9200",
          "primary-pressed": "#775D00",
  
          "shadow-default-button": "#F9B928",
          "inner-hover-button": "#F9B928",
          "inner-pressed-button": "#B58009",
  
          "primary-glow": "linear-gradient(97.93deg, #FFEAA1 0%, #FFCC15 63.99%)",
  
          /* Neutral */
          "neutral-black": "#1E2125",
          "neutral-white": "#FFFFFF",
        },
      },
    },
    plugins: [
        tailwindcssform,
    ],
};