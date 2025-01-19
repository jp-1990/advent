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

const scores = {
  A: 1,
  B: 2,
  C: 3,
  X: 1,
  Y: 2,
  Z: 3,
};

const outcomes = {
  X: 0,
  Y: 3,
  Z: 6,
};

const calcScore = (elf, you) => {
  const outcome = scores[you] - scores[elf];
  let score = 0;
  if (outcome === 1 || outcome === -2) score = scores[you] + 6;
  if (outcome === 0) score = scores[you] + 3;
  if (outcome === -1 || outcome === 2) score = scores[you];
  return score;
};

const calcScore2 = (elf, outcome) => {
  let score = 0;
  if (outcome === "Y") score = scores[elf] + outcomes[outcome];
  if (outcome === "X") {
    if (elf === "A") score = scores["Z"];
    if (elf === "B") score = scores["X"];
    if (elf === "C") score = scores["Y"];
  }
  if (outcome === "Z") {
    if (elf === "A") score = scores["Y"] + outcomes[outcome];
    if (elf === "B") score = scores["Z"] + outcomes[outcome];
    if (elf === "C") score = scores["X"] + outcomes[outcome];
  }
  return score;
};

const init = async () => {
  const lines = await processLineByLine();

  let sum1 = 0;
  let sum2 = 0;
  for (const line of lines) {
    const [elf, you] = line.split(" ");
    sum1 += calcScore(elf, you);
    sum2 += calcScore2(elf, you);
  }
  console.log(sum1);
  console.log(sum2);
};

init();
