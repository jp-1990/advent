const fs = require("fs");

function readFile() {
  const input = fs.readFileSync(__dirname + "/input.txt", { encoding: "utf8" });
  return input;
}

const TARGET = "XMAS";

const dirs = {
  r: [1, 0],
  l: [-1, 0],
  u: [0, -1],
  d: [0, 1],
  ru: [1, -1],
  rd: [1, 1],
  lu: [-1, -1],
  ld: [-1, 1],
};

function findXmas(matrix, x, y, dir, found = "") {
  const char = matrix[y]?.[x];
  if (!char) return false;

  if (char === TARGET[found.length]) {
    const newFound = found + char;
    if (newFound === TARGET) return true;

    return findXmas(matrix, x + dirs[dir][0], y + dirs[dir][1], dir, newFound);
  } else {
    return false;
  }
}

function part1() {
  const input = readFile();

  const lines = input.split("\n").filter((l) => l);
  const matrix = lines.map((l) => l.split(""));

  let sum = 0;
  for (let y = 0; y < matrix.length; y++) {
    for (let x = 0; x < matrix[y].length; x++) {
      if (matrix[y][x] === "X") {
        for (const key of Object.keys(dirs)) {
          const xmas = findXmas(matrix, x, y, key);
          if (xmas) sum++;
        }
      }
    }
  }

  return sum;
}

const pair = {
  S: "M",
  M: "S",
};

function part2() {
  const input = readFile();

  const lines = input.split("\n").filter((l) => l);
  const matrix = lines.map((l) => l.split(""));

  let sum = 0;
  for (let y = 0; y < matrix.length; y++) {
    for (let x = 0; x < matrix[y].length; x++) {
      if (matrix[y][x] === "A") {
        const lu = matrix[y - 1]?.[x - 1];
        const ld = matrix[y + 1]?.[x - 1];
        const ru = matrix[y - 1]?.[x + 1];
        const rd = matrix[y + 1]?.[x + 1];

        if (!lu || !ld || !ru || !rd) continue;

        if (pair[lu] === rd && pair[ru] === ld) {
          sum++;
        }
      }
    }
  }

  return sum;
}

function main() {
  console.log(part1());
  console.log(part2());
}

main();
