const { readfile } = require('../utils');

function checkBounds(coords, i, j) {
  if (coords[0] < 0 || coords[0] > i) return false;
  if (coords[1] < 0 || coords[1] > j) return false;
  return true
}

function part1(matrix, nodes) {
  const antinodes = {};

  for (const key of Object.keys(nodes)) {
    for (let i = 0; i < nodes[key].length; i++) {
      for (let j = i; j < nodes[key].length; j++) {
        const diff = [
          nodes[key][j][0] - nodes[key][i][0],
          nodes[key][j][1] - nodes[key][i][1],
        ]

        if (!diff[0] && !diff[1]) continue

        const prev = [
          nodes[key][i][0] - diff[0],
          nodes[key][i][1] - diff[1],
        ];

        const next = [
          nodes[key][j][0] + diff[0],
          nodes[key][j][1] + diff[1],
        ];

        if (checkBounds(prev, matrix.length - 1, matrix[0].length - 1)) {
          const str = `${prev[0]},${prev[1]}`
          antinodes[str] = true;
        }

        if (checkBounds(next, matrix.length - 1, matrix[0].length - 1)) {
          const str = `${next[0]},${next[1]}`
          antinodes[str] = true;
        }
      }
    }
  }

  console.log(Object.keys(antinodes).length);

}

function calcPrev(coords, diff) { }

function calcNext(coords, diff) { }

function part2() {
  const antinodes = {};

  for (const key of Object.keys(nodes)) {
    for (let i = 0; i < nodes[key].length; i++) {
      for (let j = i; j < nodes[key].length; j++) {
        const diff = [
          nodes[key][j][0] - nodes[key][i][0],
          nodes[key][j][1] - nodes[key][i][1],
        ]

        if (!diff[0] && !diff[1]) continue

        const prev = [
          nodes[key][i][0] - diff[0],
          nodes[key][i][1] - diff[1],
        ];

        const next = [
          nodes[key][j][0] + diff[0],
          nodes[key][j][1] + diff[1],
        ];

        if (checkBounds(prev, matrix.length - 1, matrix[0].length - 1)) {
          const str = `${prev[0]},${prev[1]}`
          antinodes[str] = true;
        }

        if (checkBounds(next, matrix.length - 1, matrix[0].length - 1)) {
          const str = `${next[0]},${next[1]}`
          antinodes[str] = true;
        }
      }
    }
  }

  console.log(Object.keys(antinodes).length);


  for (const key of Object.keys(nodes)) {
    for (let i = 0; i < nodes[key].length; i++) {
      for (let j = i; j < nodes[key].length; j++) {
        const diff = [
          nodes[key][j][0] - nodes[key][i][0],
          nodes[key][j][1] - nodes[key][i][1],
        ]

        if (!diff[0] && !diff[1]) continue

        const prev = [
          nodes[key][i][0] - diff[0],
          nodes[key][i][1] - diff[1],
        ];

        const next = [
          nodes[key][j][0] + diff[0],
          nodes[key][j][1] + diff[1],
        ];

        if (checkBounds(prev, matrix.length - 1, matrix[0].length - 1)) {
          const str = `${prev[0]},${prev[1]}`
          antinodes[str] = true;
        }

        if (checkBounds(next, matrix.length - 1, matrix[0].length - 1)) {
          const str = `${next[0]},${next[1]}`
          antinodes[str] = true;
        }
      }
    }
  }

  console.log(Object.keys(antinodes).length);

}

function main() {
  console.log('start');

  const file = readfile();

  const lines = file.split('\n');
  lines.pop();

  const nodes = {};
  const matrix = [];
  for (let i = 0; i < lines.length; i++) {
    matrix[i] = [];
    const line = lines[i].split('');
    for (let j = 0; j < line.length; j++) {
      matrix[i].push(line[j]);
      if (/^[a-zA-Z0-9]*$/.test(line[j])) {
        nodes[line[j]] ??= [];
        nodes[line[j]].push([i, j]);
      }
    }
  }

  console.log(file);
  console.log(nodes);


  part1(matrix, nodes)
  console.log('end');
}

main();
