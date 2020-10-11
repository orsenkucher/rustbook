import React from 'react';
import logo from './logo.svg';
import './App.css';
import * as wasm from "spectrum";

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <button onClick={_ => wasm.greet("Orsen")}>Click me!</button>
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
