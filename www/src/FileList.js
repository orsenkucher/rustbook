import React, { Component } from 'react'
import DragAndDrop from './DragAndDrop'

class FileList extends Component {

  state = { files: [] }

  handleDrop = (files) => {
    let fileList = this.state.files
    for (var i = 0; i < files.length; i++) {
      if (!files[i].name) return
      fileList.push(files[i].name)
    }
    this.setState({ files: fileList })
  }

  render() {
    return (
      <DragAndDrop handleDrop={this.handleDrop}>
        <div style={{ height: 300, width: 250 }}>
          {this.state.files.map((file, i) =>
            <li key={i}>
              <button onClick={this.props.onClick}>{file}</button>
            </li>
          )}
        </div>
      </DragAndDrop>
    )
  }
}

export default FileList
