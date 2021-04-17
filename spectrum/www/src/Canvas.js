import React from "react";

const Canvas = React.forwardRef((props, ref) => (
  <canvas
    ref={ref}
    className="app-canvas"
    width={props.width}
    height={props.height}
  />
));

export default Canvas
