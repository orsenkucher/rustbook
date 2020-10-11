import React from 'react';
import Canvas from './Canvas';

const App = ({ title }) =>
  <div>{title}
    <Canvas height={1000} width={1000} />
  </div>;

export default App;
