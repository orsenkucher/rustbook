import React from 'react'
import Row from './Row';

function Component({ component, setComponent, holdsArray, onCreate }) {
  const componentsMap = (iter) => {
    console.log("Mapping")
    var res = []
    while (true) {
      const t = iter.next()

      if (t == 'row') {
        const row = iter.nextRow()
        const rend = (<li key={row.path() + row.key()}>
          <Row component={row} setComponent={setComponent}></Row>
        </li>)
        res.push(rend)
      } else if (t == 'table' || t == 'array') {
        const table = iter.nextTable()
        const rend = (<li key={table.title()}>
          <Component holdsArray={t == 'array'} onCreate={() => {
            table.create();
            setComponent();
          }} component={table} setComponent={setComponent}></Component>
        </li>)
        res.push(rend)
      } else {
        break;
      }
    }
    return res
  }

  // console.log('Component(Table) title', component.title())
  return (
    <div className="app-component">
      <div>
        <b>{component.title()}</b>
        {" "}
        {(() => { if (holdsArray) return (<button onClick={onCreate}>[ <b>create</b> ]</button>) })()}
        <div style={{ height: "4px" }}></div>
      </div>

      {(() => {
        const headline = component.annotation().headline();
        if (headline) return (<div><b><i>{headline}</i></b></div>)
      })()}

      {(() => {
        const footnote = component.annotation().footnote();
        if (footnote) return (<div><i>{footnote}</i></div>)
      })()}

      <ul>{componentsMap(component.components())}</ul>
    </div>
  )
}

export default Component

