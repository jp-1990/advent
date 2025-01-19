const fs = require('fs');

function readfile() {
  const path = './input.txt';

  const file = fs.readFileSync(path, { encoding: 'utf8' })

  return file;
}

module.exports = {
  readfile
}
