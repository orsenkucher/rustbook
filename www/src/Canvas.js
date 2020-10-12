import React, { Component } from "react";
import * as spec from "spectrum";

class Canvas extends Component {
  componentDidMount() {
    spec.greet("Orsen");
  }

  render() {
    return (
      <div onDrop={e => {
        spec.ondrop(e);
      }} onDragOver={e => {
        e.preventDefault()
      }}>
        <canvas ref="canvas" width={this.props.width} height={this.props.height} />
      </div>
    )
  }
}

export default Canvas;
