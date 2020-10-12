import React, { Component } from "react";
import { greet } from "spectrum";

class Canvas extends Component {
  componentDidMount() {
    greet("Orsen");
  }

  render() {
    return (
      <div onDrop={e => {
        wasm.ondrop(e);
      }} onDragOver={e => {
        e.preventDefault()
      }}>
        <canvas ref="canvas" width={this.props.width} height={this.props.height} />
      </div>
    )
  }
}

export default Canvas;
