import * as my_test from "./my_test";

import * as wasm_demo from "./pkg/hello_world";

console.log("BOOTSTRAP: " + wasm_demo);

window.wasm_demo = wasm_demo;
