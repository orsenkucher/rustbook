import React, { useState } from 'react';
import Canvas from './Canvas';
import FileList from './FileList';
import Logs from './Logs';
import { Chart, State } from "spectrum";
// import * as spec from "spectrum";

const canvas = React.createRef()
const state = State.new()

const App = ({ title }) => {
  const [logs, setLogs] = useState(state.logs());

  return (<div className="app"><b>{title}</b>
    <div className="app-row">
      <div className="app-config">
        <FileList state={state} setLogs={setLogs} onClick={(name, text) => {
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
      <Logs logs={logs} />
    </div>
  </div >)
}

export default App
