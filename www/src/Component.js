import React from 'react'
import Row from './Row';

function Component({ component, setComponent }) {
  const componentsMap = (iter) => {
    console.log("Mapping")
    var res = []
    while (true) {
      const t = iter.next()
      // console.log("t: ", t)

      if (t == 'row') {
        // console.log("in row")
        const row = iter.nextRow()
        // console.log(row.key())
        const rend = (<li key={row.key()}>
          <Row component={row} setComponent={setComponent}></Row>
        </li>)

        res.push(rend)
      } else if (t == 'table') {
        // console.log('in table')
        const table = iter.nextTable()
        // console.log(table.title())
        const rend = (<li key={table.title()}>
          <Component component={table} setComponent={setComponent}></Component>
        </li>)
        res.push(rend)
      } else {
        // console.log('in break')
        break;
      }
    }
    return res
  }

  console.log('Component(Table) title', component.title())
  return (
    <div className="app-component">
      <div><b>{component.title()}</b></div>

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

