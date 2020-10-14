import React, { Component } from 'react'
import DragAndDrop from './DragAndDrop'

class FileList extends Component {

  state = { files: {} }

  handleDrop = async (files) => {
    const fileMap = this.state.files
    for (var i = 0; i < files.length; i++) {
      const file = files[i]
      const name = file.name
      if (!name) continue
      const ext = name.split('.').pop()
      if (ext != 'toml') continue
      const text = await file.text()
      if (name in fileMap && fileMap[name] != text) {
        if (!confirm(`Replace ${name} file?`)) {
          console.log(`Skipped ${name}`)
          continue
        }
      }
      console.log(`Added ${name}`)
      fileMap[name] = text
    }
    this.setState({ files: fileMap })
  }

  render() {
    return (
      <DragAndDrop handleDrop={this.handleDrop}>
        <div className="app-config-inner" style={{ height: 480, width: 280, border: 'dashed grey 2px', }}>
          <div>{'Config browser'}</div>
          <ol>
            {Object.keys(this.state.files).map((name, i) =>
              <li key={i}>
                <button onClick={() =>
                  this.props.onClick(name, this.state.files[name])
                }>{name}</button>
              </li>
            )}
          </ol>
        </div>
      </DragAndDrop >
    )
  }
}

export default FileList
