const fs = require("node:fs");
const readline = require("node:readline");

async function processLineByLine() {
  const fileStream = fs.createReadStream("data.txt");

  const rl = readline.createInterface({
    input: fileStream,
    crlfDelay: Infinity,
  });

  const output = [];
  let counter = 0;
  for await (const line of rl) {
    if (!line) {
      counter++;
      continue;
    }
    if (output[counter]) {
      output[counter].push(line);
    } else {
      output[counter] = [line];
    }
  }
  return output;
}

const init = async () => {
  // const elfPayloads = [[24000], [6000], [11000], [4000], [10000]]; // await processLineByLine();
  const elfPayloads = await processLineByLine();

  let max = 0;
  for (const payload of elfPayloads) {
    const sum = payload.reduce((prev, cur) => prev + +cur, 0);
    if (sum > max) max = sum;
  }

  const top3 = [0, 0, 0];
  for (const payload of elfPayloads) {
    const sum = payload.reduce((prev, cur) => prev + +cur, 0);

    for (let i = 2; i >= 0; i--) {
      if (sum > top3[i]) {
        console.log(top3, sum);
        top3.splice(i + 1, 0, sum);
        top3.shift();
        break;
      }
    }
  }
  const sum = top3.reduce((prev, cur) => prev + cur, 0);
  console.log(top3, max);
  console.log(sum);
};

init();
