import init, { run_app } from './pkg/piano_with_yew.js';
import "./css/styles.css";

async function main() {
   await init('/pkg/piano_with_yew_bg.wasm');
   run_app();
}
main()