import React from "react";

const Logs = ({ logs }) => {
  console.log(logs)
  return (
    logs.map(log => <li key={log}>{log}</li>)
  )
}

export default Logs
