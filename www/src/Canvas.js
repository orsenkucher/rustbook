import React, { Component } from "react";
import * as spec from "spectrum";

const Canvas = React.forwardRef((props, ref) => (
  <canvas
    ref={ref}
    className="Canvas"
    width={props.width}
    height={props.height}
  />
));

// class Canvas extends Component {

//   canvRef = React.createRef()

//   componentDidMount() {
//     spec.greet("Orsen")
//   }

//   render() {
//     return (
//       <div onDrop={e => {
//         spec.ondrop(e)
//         const node = this.canvRef.current
//       }} onDragOver={e => {
//         e.preventDefault()
//       }}>
//         <canvas ref={this.canvRef} width={this.props.width} height={this.props.height} />
//       </div>
//     )
//   }
// }

export default Canvas
