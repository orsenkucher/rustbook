import { saveAs } from 'file-saver';

export function writeFile(name, contents) {
  var blob = new Blob([contents], { type: "text/plain;charset=utf-8" })
  saveAs(blob, name)
}
