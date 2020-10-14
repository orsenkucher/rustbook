import React from 'react';
import Canvas from './Canvas';
import FileList from './FileList';
import { Chart } from "spectrum";
// import * as spec from "spectrum";

const canvas = React.createRef();

const App = ({ title }) =>
  <div className="app">{title}
    <div className="app-row">
      <div className="app-config">
        <FileList onClick={(name, text) => {
          console.log(name, text)
          Chart.mandelbrot(canvas.current)
        }} />
      </div>
      <div>
        <Canvas ref={canvas} height={600} width={600} />
      </div>
    </div>
  </div>;

export default App
