import React, { Component } from "react";

// const wasm = import("./fractal.wasm");
// import wa_life from "./wa_life";

// import * as wasm from "hello-wasm-pack";
// const wasm = import('hello-wasm-pack/hello-wasm-pack');

import * as wasm from "wasm-react";

// wasm.greet();


class Canvas extends Component {

  componentDidMount() {
    // wasm.then(wasm => {
    //   hahaha.greet();
    //   // const greet = hahaha.greet;
    //   // greet();
    // });
    // hahaha.greet("Kke");
    console.log("hahaha");
    wasm.greet("Bogdan");

    // wa_life.greet("Bogdan");
  }

  render() {
    return (
      <canvas ref="canvas" width={this.props.width} height={this.props.height} />
    )
  }
}

export default Canvas;
