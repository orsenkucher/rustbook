import React, { Component } from "react";
// import * as wasm from "wasm-react";
// import { Chart } from "../../pkg/wasm_react";
import { Chart } from "wasm-react";
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
      <canvas ref="canvas" width={this.props.width} height={this.props.height} onClick={() => {
        Chart.mandelbrot(this);
        // Chart.testfn(this);
      }} />
    )
  }
}

export default Canvas;
