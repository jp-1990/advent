const { readfile } = require("../utils");

const movements = {
  0: [-1, 0],
  1: [0, 1],
  2: [1, 0],
  3: [0, -1],
};

function move(coords, dir) {
  return {
    0: [coords[0] + movements[0][0], coords[1] + movements[0][1]],
    1: [coords[0] + movements[1][0], coords[1] + movements[1][1]],
    2: [coords[0] + movements[2][0], coords[1] + movements[2][1]],
    3: [coords[0] + movements[3][0], coords[1] + movements[3][1]],
  }[dir];
}

function getLocationValue(matrix, coords) {
  return matrix[coords[0]]?.[coords[1]];
}

/**
 * @param matrix{number[][]}
 * @param coords{[number,number]}
 */
function walk(matrix, coords, count = 0, seen = {}) {
  const location = matrix[coords[0]][coords[1]];
  if (location === 9) {
    return 1;
  }

  const up = getLocationValue(matrix, move(coords, 0));
  const right = getLocationValue(matrix, move(coords, 1));
  const down = getLocationValue(matrix, move(coords, 2));
  const left = getLocationValue(matrix, move(coords, 3));

  const values = [up, right, down, left];

  let noValidMove = true;
  let count_ = 0;
  for (let i = 0; i < values.length; i++) {
    const value = values[i];
    if (value === undefined) continue;
    if (value === location + 1) {
      noValidMove = false;
      const test = walk(matrix, move(coords, i), count, seen);
      count_ += test;
    }
  }

  if (noValidMove) return 0;
  return count_;
}

/**
 * @param matrix{number[][]}
 * @param coords{[number,number]}
 */
function walk2(matrix, coords, count = 0, seen = {}) {
  const location = matrix[coords[0]][coords[1]];
  if (location === 9) {
    if (!seen[`${coords[0]},${coords[1]}`]) {
      seen[`${coords[0]},${coords[1]}`] = true;
      return 1;
    }
    return 0;
  }

  const up = getLocationValue(matrix, move(coords, 0));
  const right = getLocationValue(matrix, move(coords, 1));
  const down = getLocationValue(matrix, move(coords, 2));
  const left = getLocationValue(matrix, move(coords, 3));

  const values = [up, right, down, left];

  let noValidMove = true;
  let count_ = 0;
  for (let i = 0; i < values.length; i++) {
    const value = values[i];
    if (value === undefined) continue;
    if (value === location + 1) {
      noValidMove = false;
      const test = walk(matrix, move(coords, i), count, seen);
      count_ += test;
    }
  }

  if (noValidMove) return 0;
  return count_;
}

/**
 * @param matrix{number[][]}
 */
function part1(matrix) {
  // trail should be as long as possible
  // only u/d/l/r
  // start at 0, end at 9, increment by 1 each step
  // trailhead = 0
  // score = number of 9s reachable from a trailhead
  // find the score of all the trailheads
  // sum the scores

  let sum = 0;
  let break_ = false;
  for (let i = 0; i < matrix.length; i++) {
    if (break_) break;
    for (let j = 0; j < matrix[i].length; j++) {
      if (break_) break;
      const value = matrix[i][j];
      if (value === 0) {
        const trail = walk(matrix, [i, j]);
        sum += trail;
        // break_ = true;
      }
    }
  }

  console.log(sum);
}

/**
 * @param matrix{number[][]}
 */
function part2(matrix) {
  let sum = 0;
  for (let i = 0; i < matrix.length; i++) {
    for (let j = 0; j < matrix[i].length; j++) {
      const value = matrix[i][j];
      if (value === 0) {
        const trail = walk2(matrix, [i, j]);
        sum += trail;
      }
    }
  }

  console.log(sum);
}

function main() {
  console.time("run");

  const input = readfile();

  const matrix = input.split("\n").map((line) => line.split("").map(Number));
  matrix.pop();
  part1(matrix);
  part2(matrix);

  console.timeEnd("run");
}

main();
