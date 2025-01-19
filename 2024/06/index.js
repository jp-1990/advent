const { readfile } = require('../utils');

const move = {
  u: [-1, 0],
  d: [1, 0],
  l: [0, -1],
  r: [0, 1],
}

const nextDir = {
  u: 'r',
  r: 'd',
  d: 'l',
  l: 'u'
}

class Point {
  dir = undefined;
  coords = [undefined, undefined]
  str = undefined;

  constructor(i, j, dir) {
    this.coords = [i, j]
    this.dir = dir;
    this.str = this.toString();
  }

  next(dir) {
    const i = this.coords[0] + move[dir][0];
    const j = this.coords[1] + move[dir][1];

    return new Point(i, j, dir)
  }

  value(matrix) {
    return matrix[this.coords[0]]?.[this.coords[1]];
  }

  isEqual(point) {
    return this.coords[0] === point.coords[0] && this.coords[1] === point.coords[1];
  }

  toString() {
    return `${this.coords[0]},${this.coords[1]}`
  }
}

function walk(matrix, point, dir) {
  const next = point.next(dir);
  if (!next.value(matrix)) {
    return undefined;
  }
  if (next.value(matrix) === '#') {
    return point.next(nextDir[dir]);
  }
  return next;
}

function walkLoop(matrix, start, startDir) {
  let cur = start;
  let dir = nextDir[startDir];
  const moves = { l: 0, r: 0, u: 0, d: 0 }
  while (true) {
    const next = walk(matrix, cur, dir);

    if (!next) break;
    moves[next.dir]++;

    if (moves[dir] > 0 && next.isEqual(start)) break;

    cur = next;
    dir = next.dir
  }

  if ((moves['l'] > 0 || moves['u'] > 0) && moves['l'] === moves['r'] && moves['u'] === moves['d']) {
    return true

  }

  return false
}

function part1(matrix, start) {
  const seen = {};

  let count = 0;
  let cur = start;
  let dir = 'u';
  while (true) {
    const next = walk(matrix, cur, dir);

    if (!next) break

    if (!seen[next.str]) {
      count++;
      seen[next.str] = 1;
    }

    cur = next;
    dir = next.dir
  }

  console.log(count);
}

function part2(matrix, start) {
  let count = 0;
  let cur = start;
  let dir = 'u';
  while (true) {
    if (walkLoop(matrix, cur, dir)) count++;

    const next = walk(matrix, cur, dir);
    if (!next) break

    cur = next;
    dir = next.dir
  }

  console.log(count);

}


function main() {

  const file = readfile();
  const lines = file.split('\n');
  lines.pop();

  let start = [undefined, undefined];
  const matrix = [];
  for (let i = 0; i < lines.length; i++) {
    matrix[i] = []
    const line = lines[i].split('');
    for (let j = 0; j < line.length; j++) {
      if (line[j] === '^') start = [i, j];
      matrix[i][j] = line[j]
    }
  }


  console.log('start')
  // console.log(file);

  // part1(matrix, new Point(start[0], start[1]));
  part2(matrix, new Point(start[0], start[1]));

  console.log('end')
};

main();
