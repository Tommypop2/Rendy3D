import { defineConfig } from "vite";
import pluginWASM from "vite-plugin-wasm";
export default defineConfig({
	base: "/Rendy3D/",
	plugins: [pluginWASM()] });