const fs = require("fs");

function readFile() {
  const input = fs.readFileSync(__dirname + "/input.txt", { encoding: "utf8" });
  return input;
}

function part1() {
  const input = readFile();

  const regex = new RegExp(/mul\(([0-9]{1,3}),([0-9]{1,3})\)/g);
  const results = input.match(regex);

  let sum = 0;
  for (const result of results) {
    const [a, b] = result.replace("mul(", "").replace(")", "").split(",");
    sum += +a * +b;
  }

  return sum;
}

function part2() {
  const input = readFile();

  const splitInput = input.split(/don't\(\)/g);

  let doString = "";
  for (let i = 0; i < splitInput.length; i++) {
    if (i === 0) {
      doString += splitInput[i];
      continue;
    }
    const split = splitInput[i].split(/do\(\)/g);
    split.shift();
    doString += split.join("");
  }

  const regex = new RegExp(/mul\(([0-9]{1,3}),([0-9]{1,3})\)/g);
  const results = doString.match(regex);

  let sum = 0;
  for (const result of results) {
    const [a, b] = result.replace("mul(", "").replace(")", "").split(",");
    sum += +a * +b;
  }

  return sum;
}

function main() {
  part1();
  part2();
}

main();
