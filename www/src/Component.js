import React from 'react'

function Component({ component }) {
  console.log('Component title', component.title())
  return (
    <div className="app-component">
      <div>title: {component.title()}</div>
      <div>headline: {component.annotation().headline()}</div>
      <div>footnote: {component.annotation().footnote()}</div>
      {/* <ol>
        {props.components.map((compnent, i) =>
          <li key={compnent + i}>
            <Component props={component}></Component>
          </li>
        )}
      </ol> */}
    </div>
  )
}

export default Component

