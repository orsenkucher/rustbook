import React from 'react'

function Row({ component }) {
  console.log('Component(Row) key', component.key())
  return (
    <div className="app-component">
      <div>key: {component.key()}</div>
      <div>value: {component.value()}</div>
      <div>headline: {component.annotation().headline()}</div>
      <div>footnote: {component.annotation().footnote()}</div>
    </div>
  )
}

export default Row

