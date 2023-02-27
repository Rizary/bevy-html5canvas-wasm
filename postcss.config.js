import postcssPurgecss from "@fullhuman/postcss-purgecss";
import autoprefixer from "autoprefixer";
import cssnanoPlugin from "cssnano";
import postcssImport from "postcss-import";
import postcssNested from "postcss-nested";
import tailwindcss from "tailwindcss";
import tailwindConfig from './tailwind.config.js';

const env = process.env.NODE_ENV;
const isProd = env === "production";


const purgecss = postcssPurgecss({

    // Specify the paths to all of the template files in your project
    content: ['./src/*.rs', './src/**/*.rs', './js/wallet.js'],

    keyframes: true,

    // Include any special characters you're using in this regular expression
    defaultExtractor: content => content.match(/[A-Za-z0-9-_:/]+/g) || []
})

export default {
    plugins: [
        ...(isProd ? [purgecss] : []),
        postcssImport('postcss-import'),
        tailwindcss(tailwindConfig),
        autoprefixer,
        ...(isProd ? [cssnanoPlugin] : []),
        postcssNested,
    ]
}