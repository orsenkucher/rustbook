import React, { Component } from "react";
// import { Chart } from "../../pkg/wasm_react";
import { Chart } from "wasm-react";
import * as wasm from "wasm-react";
// class Chart { }

class Canvas extends Component {
  componentDidMount() {
    // wasm.greet("Bogdan");
    // console.log(Chart.mandelbrot(this));
    console.log(Chart);
    console.log(Chart.mandelbrot);
    // console.log(Chart.testfn);
    // let chart = Chart.mandelbrot(this);
    // console.log(chart);
  }

  render() {
    return (
      <div onDrop={e => {
        wasm.ondrop(e);
      }} onDragOver={e => {
        e.preventDefault()
      }}>
        <canvas ref="canvas" width={this.props.width} height={this.props.height} onClick={() => {
          // let my_ref = this.myRef.current;
          let my_ref = this.refs["canvas"];
          console.log(my_ref);
          Chart.mandelbrot(my_ref);
          // Chart.testfn(this);
        }} />
      </div>
    )
  }
}

export default Canvas;
