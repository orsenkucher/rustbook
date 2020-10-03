import React from 'react';
import ReactDOM from 'react-dom';

import App from './App';

const title = 'My yes Minimal React Webpack Babel Setup';

// export function setup(WasmChart) {
//   console.log("running setup");
//   Chart = WasmChart;
// }

ReactDOM.render(
  <App title={title} />,
  document.getElementById('app')
);

module.hot.accept();
