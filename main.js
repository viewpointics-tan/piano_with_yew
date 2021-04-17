import init, { run_app } from './pkg/yew_wasm_pack_tailwind.js';
import "./css/styles.css";

async function main() {
   await init('/pkg/yew_wasm_pack_tailwind_bg.wasm');
   run_app();
}
main()