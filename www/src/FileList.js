import React, { useState } from 'react'
import DragAndDrop from './DragAndDrop'

function FileList(props) {
  // const [_, setState] = useState(props.state);
  const handleDrop = async (files) => {
    const fileMap = props.state.files()
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
    console.log(fileMap)
    props.state.log("JS: Updated files")
    props.state.setFiles(fileMap)
    props.setLogs(props.state.logs())
    // setState(() => { })
    console.log(props.state.logs())
    console.log(props.state.files())
  }

  return (
    <DragAndDrop handleDrop={handleDrop}>
      <div className="app-config-inner">
        <div>Config browser</div>
        <ol>
          {Object.keys(props.state.files()).map((name, i) =>
            <li key={i}>
              <button onClick={() =>
                props.onClick(name, props.state.files()[name])
              }>{name}</button>
            </li>
          )}
        </ol>
      </div>
    </DragAndDrop >
  )
}

export default FileList
