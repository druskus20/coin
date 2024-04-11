// place files you want to import through the `$lib` alias in this fold
//

import init, { run }  from "./coin_core.js"
//import init, { run, log_thing }  from "./pkg/surreal_to_wasm_test.js";
export async function init_and_run() {
  await init();
  try {
    let a = await run();
    console.log("Rust returned: ", a);
  } catch (e) {
    console.error("Rust error catched in JS");
  }
}


