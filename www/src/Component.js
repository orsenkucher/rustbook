import React from 'react'
import Row from './Row';

function Component({ component }) {
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
          <Row component={row}></Row>
        </li>)

        res.push(rend)
      } else if (t == 'table') {
        // console.log('in table')
        const table = iter.nextTable()
        // console.log(table.title())
        const rend = (<li key={table.title()}>
          <Component component={table}></Component>
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
      <div>title: {component.title()}</div>
      <div>headline: {component.annotation().headline()}</div>
      <div>footnote: {component.annotation().footnote()}</div>
      <ol>
        {
          componentsMap(component.components())
          // props.components.map((component, i) =>
          //   <li key={component + i}>
          //     <Component props={component}></Component>
          //   </li>
          // )
        }
      </ol>
    </div>
  )
}

export default Component

