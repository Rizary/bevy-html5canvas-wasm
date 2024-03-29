import commonjs from "@rollup/plugin-commonjs";
import nodeResolve from "@rollup/plugin-node-resolve";
import rust from "@wasm-tool/rollup-plugin-rust";
import livereload from "rollup-plugin-livereload";
import serve from "rollup-plugin-serve";

export default {
    input: {
        index: 'Cargo.toml'
    },
    output: {
        dir: "devhtml/js",
        format: "cjs",
        sourcemap: true,
        chunkFileNames: "[name].js",
        assetFileNames: "assets/[name][extname]"
    },
    plugins: [
        rust({
            serverPath: "/js/",
            debug: true,
            verbose: true,
            watchPatterns: ["src/**"],
            cargoArgs: ["--features", "develop"],
        }),
        nodeResolve(),
        commonjs(),
        // injectProcessEnv(getEnv()),
        serve({
            contentBase: 'devhtml',
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
            watch: 'devhtml',
            verbose: true
        })
    ],
    watch: {
        clearScreen: false
    }
};
