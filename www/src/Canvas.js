import React, { Component } from "react";
import * as wasm from "spectrum";


class Canvas extends Component {
  componentDidMount() {
    // wasm.greet("Bogdan");
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
