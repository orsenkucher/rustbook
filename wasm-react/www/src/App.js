import React from 'react';
import Canvas from './canvas';

const App = ({ title }) =>
  <div>{title}
    <Canvas height={500} width={500} />
  </div>;

export default App;
