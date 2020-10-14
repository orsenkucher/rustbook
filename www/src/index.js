import React from 'react';
import ReactDOM from 'react-dom';
import App from './App';
import './index.css';


const title = 'Simulation of hardware spectrum';

ReactDOM.render(
  <App title={title} />,
  document.getElementById('app')
);

module.hot.accept();
