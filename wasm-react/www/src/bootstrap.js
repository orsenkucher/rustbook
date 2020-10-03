// A dependency graph that contains any wasm must all be imported
// asynchronously. This `bootstrap.js` file does the single async import, so
// that no one else needs to worry about it again.
import("./index.js")
  .catch(e => console.error("Error importing `index.js`:", e));

// init();

// async function init() {
//   console.log("branch 0");
//   if (typeof process == "object") {
//     console.log("branch 1");
//     // We run in the npm/webpack environment.
//     const [{ Chart }, { main, setup }] = await Promise.all([
//       import("wasm-react"),
//       import("./index.js"),
//     ]);
//     setup(Chart);
//     main();
//   } else {
//     console.log("branch 2");
//     const [{ Chart, default: init }, { main, setup }] = await Promise.all([
//       import("../../pkg/wasm_react.js"),
//       import("./index.js"),
//     ]);
//     await init();
//     setup(Chart);
//     main();
//   }
// }
