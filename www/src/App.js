import React from 'react';
import Canvas from './Canvas';
import FileList from './FileList';
import { Chart } from "spectrum";
// import * as spec from "spectrum";

const canvas = React.createRef();

const App = ({ title }) =>
  <div>{title}
    <div>
      <FileList onClick={(name, text) => {
        console.log(name, text)
        Chart.mandelbrot(canvas.current);
      }} />
    </div>
    <Canvas ref={canvas} height={800} width={800} />
  </div>;

export default App
