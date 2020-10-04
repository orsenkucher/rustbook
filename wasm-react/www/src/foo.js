import { saveAs } from 'file-saver';


export function write_file(contents) {
  // return fs.readFileSync(path, { encoding: "utf8" });
  var blob = new Blob([contents], { type: "text/plain;charset=utf-8" });
  saveAs(blob, "hello world.txt");
}
