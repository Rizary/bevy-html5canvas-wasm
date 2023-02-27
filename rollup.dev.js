import commonjs from "@rollup/plugin-commonjs";
import nodeResolve from "@rollup/plugin-node-resolve";
import rust from "@wasm-tool/rollup-plugin-rust";
import copy from 'rollup-plugin-copy';
import livereload from "rollup-plugin-livereload";
import serve from "rollup-plugin-serve";

export default {
    input: {
        index: 'Cargo.toml'
    },
    output: {
        dir: "dist/static",
        format: "iife",
        sourcemap: true,
        chunkFileNames: "[name].js",
        assetFileNames: "assets/[name][extname]"
    },
    plugins: [
        copy({
            targets: [
                { src: 'index.html', dest: 'dist' },
                { src: 'static/*', dest: 'dist/static' },
            ]
        }),
        rust({
            serverPath: "/static/",
            debug: true,
            verbose: true,
            watchPatterns: ["src/**", "./index.html", "./Cargo.toml"],
            cargoArgs: ["--features", "develop", "--profile", "dev"],
            wasmOptArgs: ["-Oz", "gc-sections"],
        }),
        nodeResolve(),
        commonjs(),
        // injectProcessEnv(getEnv()),
        serve({
            contentBase: 'dist',
            open: true,
            verbose: true,
            debug: false,
            port: 8189,
            headers: {
                'Access-Control-Allow-Origin': '*',
                "Content-Type": "application/wasm",
            },
            // historyApiFallback: true,
        }),

        livereload({
            watch: 'dist',
            verbose: true
        })
    ],
    watch: {
        clearScreen: false
    }
};
