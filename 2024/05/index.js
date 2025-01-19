const fs = require("fs");

function readFile() {
  const input = fs.readFileSync(__dirname + "/input.txt", { encoding: "utf8" });
  return input;
}

const mockInput =
  "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47";

function part1() {
  const [rules, inputs] = mockInput.split("\n\n");

  const rulesHash = {};
  for (const rule of rules.split("\n")) {
    const [x, y] = rule.split("|");
    rulesHash[x] ??= [];
    rulesHash[x].push(y);
  }

  const correct = [];
  for (const input of inputs.trim().split("\n")) {
    const splitInput = input.split(",");
    const inputHash = {};
    for (let i = 0; i < splitInput.length; i++) {
      inputHash[splitInput[i]] = i;
    }

    let ordered = true;
    for (let i = 0; i < splitInput.length; i++) {
      for (const rule of rulesHash[splitInput[i]] ?? []) {
        if (inputHash[rule] <= i) ordered = false;
      }
    }

    if (ordered) {
      correct.push(splitInput[Math.floor(splitInput.length / 2)]);
    }
  }

  let sum = 0;
  for (const c of correct) {
    sum += +c;
  }

  return sum;
}

function part2() {
  const [rules, inputs] = readFile().split("\n\n");

  const rulesHash = {};
  for (const rule of rules.split("\n")) {
    const [x, y] = rule.split("|");
    rulesHash[x] ??= [];
    rulesHash[x].push(y);
  }

  const incorrect = [];
  for (const input of inputs.trim().split("\n")) {
    const splitInput = input.split(",");

    let ordered = true;
    const output = [...splitInput];
    for (let i = 0; i < splitInput.length; i++) {
      for (const rule of rulesHash[output[i]] ?? []) {
        const currentPositions = {};
        for (let i = 0; i < output.length; i++) {
          currentPositions[output[i]] = i;
        }

        if (currentPositions[rule] <= i) {
          ordered = false;
          output.splice(currentPositions[rule], 1);
          output.splice(i + 1, 0, rule);
          i = 0;
        }
      }
    }

    if (!ordered) {
      incorrect.push(output[Math.floor(output.length / 2)]);
    }
  }

  let sum = 0;
  for (const c of incorrect) {
    sum += +c;
  }

  return sum;
}

function main() {
  console.log(part1());
  console.log(part2());
}

main();
