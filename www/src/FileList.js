import React from 'react'
import DragAndDrop from './DragAndDrop'

function FileList(props) {
  const handleDrop = async (files) => {
    var modified = {}
    for (const [key, value] of props.state.files()) {
      modified[key] = value.modified
    }

    var updated = 0
    for (var i = 0; i < files.length; i++) {
      const file = files[i]
      const name = file.name
      if (!name) continue
      const ext = name.split('.').pop()
      if (ext != 'toml') continue
      const text = await file.text()
      if (name in modified) {
        if (modified[name] == text) continue
        if (!confirm(`Replace ${name} file?`)) {
          props.state.log(`Skipped ${name}`)
          continue
        }
      }
      updated++
      console.log(`Added ${name}`)
      modified[name] = text
    }
    console.log(modified)
    props.state.log(`Updated ${updated} files`)
    props.state.setFiles(modified)
    props.setLogs(props.state.logs())
    console.log(props.state.files())
  }

  const files = props.state.files()
  for (const [key, value] of files) {
    console.log(key, value.isModified())
  }

  return (
    <DragAndDrop handleDrop={handleDrop}>
      <div className="app-config-inner">
        <div>Config browser</div>
        <ol>
          {[...files].map(x => x[0]).map((name, i) =>
            <li key={i}>
              {files.get(name).isModified() ? "⚙️" : ""}
              <button onClick={() => props.onClick(name)}>{name}</button>
              {" "}
              <button onClick={() => props.onDownload(name)}>⤓</button>
              <div style={{ height: "4px" }}></div>
            </li>
          )}
        </ol>
      </div>
    </DragAndDrop >
  )
}

export default FileList
