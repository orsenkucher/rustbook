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
    <div className="app-component">
      <div>key: {component.key()}</div>
      {/* <div>value: {component.value()} */}
      <div>value:
        <input type="text" value={value} onChange={valueChange} />
        {/* <input type="text" value={component.value()} onChange={valueChange} /> */}
      </div>
      <div>headline: {component.annotation().headline()}</div>
      <div>footnote: {component.annotation().footnote()}</div>
    </div>
  )
}

export default Row

