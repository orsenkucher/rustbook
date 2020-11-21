import React, { useState, useEffect } from 'react';
import Canvas from './Canvas';
import FileList from './FileList';
import Logs from './Logs';
import { State } from "spectrum";
import Component from './Component';

const canvas = React.createRef()
const state = State.new()

const App = ({ title }) => {
  const [logs, setLogs] = useState(state.logs());
  const [component, setComponent] = useState(state.component());

  useEffect(() => {
    document.title = `Spectrum: ${[...state.files()].length} open`;
  });

  return (<div className="app"><b>{title}</b>
    <div className="app-row">
      <div className="app-config">
        <FileList
          state={state}
          setLogs={setLogs}
          onClick={name => {
            state.log(`Plotting ${name}`)
            try {
              state.handle(canvas.current, name)
            } catch { }
            setComponent(state.component())
            console.log('App title', state.component().title())
            setLogs(state.logs())
          }}
          onDownload={name => {
            state.log(`Downloading ${name}`)
            state.download(name)
            setLogs(state.logs())
          }}
          setComponent={() => setComponent(state.component())}
        />
      </div>
      <div>
        <Canvas ref={canvas} height={800} width={800} />
      </div>
      <div className="app-fields">
        <div>Fields editor</div>
        <Component component={component} setComponent={() => {
          state.evaluate()
          setLogs(state.logs())
          setComponent(state.component())
        }}></Component>
      </div>
    </div>
    <div className="app-logs">
      <Logs logs={logs} />
    </div>
  </div >)
}

export default App
