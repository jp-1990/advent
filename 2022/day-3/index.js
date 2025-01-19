const fs = require("node:fs");
const readline = require("node:readline");

async function processLineByLine() {
  const fileStream = fs.createReadStream("data.txt");

  const rl = readline.createInterface({
    input: fileStream,
    crlfDelay: Infinity,
  });

  const output = [];
  for await (const line of rl) {
    output.push(line);
  }
  return output;
}

const priority = {
  a: 1,
  b: 2,
  c: 3,
  d: 4,
  e: 5,
  f: 6,
  g: 7,
  h: 8,
  i: 9,
  j: 10,
  k: 11,
  l: 12,
  m: 13,
  n: 14,
  o: 15,
  p: 16,
  q: 17,
  r: 18,
  s: 19,
  t: 20,
  u: 21,
  v: 22,
  w: 23,
  x: 24,
  y: 25,
  z: 26,
  A: 27,
  B: 28,
  C: 29,
  D: 30,
  E: 31,
  F: 32,
  G: 33,
  H: 34,
  I: 35,
  J: 36,
  K: 37,
  L: 38,
  M: 39,
  N: 40,
  O: 41,
  P: 42,
  Q: 43,
  R: 44,
  S: 45,
  T: 46,
  U: 47,
  V: 48,
  W: 49,
  X: 50,
  Y: 51,
  Z: 52,
};

const part1 = (lines) => {
  let sum = 0;
  for (const line of lines) {
    const a = line.slice(0, line.length / 2);
    const b = line.slice(line.length / 2);

    let item = "";
    for (const char of a.split("")) {
      if (b.split("").some((e) => e === char)) {
        item = char;
        break;
      }
    }

    const itemPriority = priority[item];
    sum += itemPriority;
  }
  console.log(sum);
};

const part2 = (lines) => {
  let sum = 0;

  for (let i = 1; i <= lines.length; i++) {
    if (i % 3 === 0) {
      let item = "";
      for (const char of lines[i - 1].split("")) {
        if (
          lines[i - 2].split("").some((e) => e === char) &&
          lines[i - 3].split("").some((e) => e === char)
        ) {
          item = char;
          break;
        }
      }
      const itemPriority = priority[item];
      sum += itemPriority;
    }
  }

  console.log(sum);
};

const init = async () => {
  const lines = await processLineByLine();

  //   part1(lines);
  part2(lines);
};

init();
