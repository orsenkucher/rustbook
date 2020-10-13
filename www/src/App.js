import React from 'react';
import Canvas from './Canvas';
import FileList from './FileList';
import { Chart } from "spectrum";
// import * as spec from "spectrum";

const canvas = React.createRef();

const App = ({ title }) =>
  <div>{title}
    <div>
      <FileList onClick={() => {
        console.log("click")
        Chart.mandelbrot(canvas.current);
      }} />
    </div>
    <Canvas ref={canvas} height={1000} width={1000} />
  </div>;

export default App
