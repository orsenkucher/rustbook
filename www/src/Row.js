import React, { useState } from 'react'

function Row({ component, setComponent }) {
  const [value, setValue] = useState(component.value());
  const valueChange = (event) => {
    console.log(event.target.value)
    component.modifyValue(event.target.value)
    setValue(component.value())
    setComponent()
  }

  console.log('Component(Row) key', component.key())
  return (
    <div className="app-component-row">
      {(() => {
        const headline = component.annotation().headline();
        if (headline) return (<div><b><i>{headline}</i></b></div>)
      })()}

      <div>{component.key()}: {<input type="text" value={value} onChange={valueChange} />}
        {component.isModified() ? "⚙️" : ""} {(() => {
          const footnote = component.annotation().footnote();
          if (footnote) return (<i>{footnote}</i>)
        })()}
      </div>
    </div >
  )
}

export default Row

