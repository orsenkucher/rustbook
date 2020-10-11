import React from 'react';
import ReactDOM from 'react-dom';
import App from './App';

import * as wasm from "spectrum";

wasm.greet();
// wasm.greet("Orsen");

const title = 'Spectrum';

ReactDOM.render(
  <App title={title} />,
  document.getElementById('app')
);

module.hot.accept();
