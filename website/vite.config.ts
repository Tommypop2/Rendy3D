import { defineConfig } from "vite";
import pluginWASM from "vite-plugin-wasm";
export default defineConfig({ plugins: [pluginWASM()] });