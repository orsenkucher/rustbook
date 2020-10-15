import React from 'react';
import Canvas from './Canvas';
import FileList from './FileList';
import Logs from './Logs';
import { Chart } from "spectrum";
// import * as spec from "spectrum";

const canvas = React.createRef();

const App = ({ title }) =>
  <div className="app"><b>{title}</b>
    <div className="app-row">
      <div className="app-config">
        <FileList onClick={(name, text) => {
          console.log(name, text)
          Chart.mandelbrot(canvas.current)
        }} />
      </div>
      <div>
        <Canvas ref={canvas} height={800} width={800} />
      </div>
      <div className="app-fields">
        <div>Fields editor</div>
      </div>
    </div>
    <div className="app-logs">
      <Logs logs={[1, 2, 3, 4, 5, 6, 7]} />
    </div>
  </div >;

export default App
