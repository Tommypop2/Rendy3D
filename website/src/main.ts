import './style.css'
import { start_web } from "../../web/pkg";
let canvas = document.getElementById("winit-canvas") as HTMLCanvasElement;
start_web(canvas);
