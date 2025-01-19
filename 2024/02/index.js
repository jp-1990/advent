const fs = require("fs");

function readFile() {
  const input = fs.readFileSync(__dirname + "/input.txt", { encoding: "utf8" });
  return input;
}

function part1() {
  // const reports = [
  //   [7, 6, 4, 2, 1],
  //   [1, 2, 7, 8, 9],
  //   [9, 7, 6, 2, 1],
  //   [1, 3, 2, 4, 5],
  //   [8, 6, 4, 1, 3],
  //   [1, 3, 6, 7, 9],
  // ];

  const reports = [];

  const input = readFile();
  const splitInput = input.split(/\n/g);
  for (const i of splitInput) {
    const line = i.split(/\s/g).filter((e) => e);
    if (!line.length) continue;
    reports.push(line.map((e) => +e).filter((e) => e));
  }

  // max diff 3 - min diff 1
  // all increasing or all decreasing
  // sum total

  let sum = 0;
  for (const r of reports) {
    let dir = 0;
    let prevN = 0;

    let safe = true;
    for (let i = 0; i < r.length; i++) {
      if (i === 0) {
        prevN = r[i];
        continue;
      }
      if (i === 1 && r[i] < prevN) dir = -1;
      if (i === 1 && r[i] > prevN) dir = 1;

      const dif = Math.abs(r[i] - prevN);
      if (dif > 3 || dif < 1) safe = false;

      if (r[i] === prevN) safe = false;
      if (r[i] > prevN && dir === -1) safe = false;
      if (r[i] < prevN && dir === 1) safe = false;

      prevN = r[i];
      if (safe === false) break;
    }
    // console.log(safe, "\n");

    if (safe) sum += 1;
  }

  console.log(sum);
}

function checkReport(r) {
  let dir = 0;
  let prevN = 0;
  let failedIndex = undefined;
  let safe = true;

  for (let i = 0; i < r.length; i++) {
    if (i === 0) {
      prevN = r[i];
      continue;
    }
    if (i === 1 && r[i] < prevN) dir = -1;
    if (i === 1 && r[i] > prevN) dir = 1;

    const dif = Math.abs(r[i] - prevN);
    if (dif > 3 || dif < 1) safe = false;

    if (r[i] === prevN) safe = false;
    if (r[i] > prevN && dir === -1) safe = false;
    if (r[i] < prevN && dir === 1) safe = false;

    prevN = r[i];
    if (safe === false && failedIndex === undefined) failedIndex = i;
  }

  return { safe, failedIndex };
}

function part2() {
  const reports = [];

  const input = readFile();
  const splitInput = input.split(/\n/g);
  for (const i of splitInput) {
    const line = i.split(/\s/g).filter((e) => e);
    if (!line.length) continue;
    reports.push(line.map((e) => +e).filter((e) => e));
  }

  // max diff 3 - min diff 1
  // all increasing or all decreasing
  // also safe if removing one level passes previous checks
  // sum total passing

  let sum = 0;
  for (const report of reports) {
    let safe = true;
    const check = checkReport(report);
    safe = check.safe;

    if (!safe) {
      for (let i = 0; i < report.length; i++) {
        const r = [...report];
        r.splice(i, 1);
        const check = checkReport(r);
        if (check.safe) {
          safe = true;
          break;
        }
      }
    }

    if (safe) sum++;
  }

  return sum;
}

function main() {
  part1();
  part2();
}

main();
