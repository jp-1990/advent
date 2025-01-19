const fs = require("fs");

function readFile() {
  const input = fs.readFileSync(__dirname + "/input.txt", { encoding: "utf8" });
  return input;
}

function part1() {
  const left = [];
  const right = [];

  const input = readFile();
  const splitInput = input.split(/\n/g);
  for (const i of splitInput) {
    const [a, b] = i.split(/\s/g).filter((e) => e);
    a && left.push(a);
    b && right.push(b);
  }

  left.sort();
  right.sort();

  const diff = left.map((e, i) => {
    return Math.abs(e - right[i]);
  });

  let output = 0;
  for (const n of diff) {
    output += n;
  }

  console.log(output);
}

function part2() {
  // how often each number from the left list appears in right list
  // for each left multiply by frequency
  // sum left list

  const left = [];
  const right = [];

  const input = readFile();
  const splitInput = input.split(/\n/g);
  for (const i of splitInput) {
    const [a, b] = i.split(/\s/g).filter((e) => e);
    a && left.push(+a);
    b && right.push(+b);
  }

  const rightHash = {};
  for (const n of right) {
    rightHash[n] ??= 0;
    rightHash[n] += 1;
  }

  let sum = 0;
  for (const n of left) {
    const freq = rightHash[n] ?? 0;
    sum += freq * n;
  }

  return sum;
}

function main() {
  part1();
  part2();
}

main();
