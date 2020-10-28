import React from "react";

const Logs = ({ logs }) => {
  return (
    logs.map((log, i) => <li key={log + i}>{log}</li>)
  )
}

export default Logs
