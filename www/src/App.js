import React from 'react';
import Canvas from './Canvas';
import FileList from './FileList';

const App = ({ title }) =>
  <div>{title}
    <div>
      <FileList />
    </div>
    {/* <Canvas height={1000} width={1000} /> */}
  </div>;

export default App
