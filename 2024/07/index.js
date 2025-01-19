const { readfile } = require('../utils');


const operators = ['+', '*']

function calculate(answer, inputs, prev) {
  if (prev > answer) return 0;
  if (inputs.length === 1 && prev === answer) return prev;
  if (inputs.length === 1) return 0;


  const multiply = calculate(answer, inputs.slice(1), prev * inputs[1]);
  const add = calculate(answer, inputs.slice(1), prev + inputs[1]);

  return multiply === answer ? multiply : add;
}

function part1(input) {
  console.log('part 1:')

  let count = 0;
  for (let i = 0; i < input.length; i++) {
    const result = calculate(input[i][0], input[i][1], input[i][1][0])
    count += result
  }

  console.log(count)
}

function calculate2(answer, inputs, prev) {
  if (prev > answer) return 0;
  if (inputs.length === 1 && prev === answer) return prev;
  if (inputs.length === 1) return 0;


  const multiply = calculate2(answer, inputs.slice(1), prev * inputs[1]);
  const add = calculate2(answer, inputs.slice(1), prev + inputs[1]);
  const concat = calculate2(answer, inputs.slice(1), +`${prev}${inputs[1]}`);

  return multiply === answer ? multiply : add === answer ? add : concat;
}

function part2(input) {
  console.log('part 2:')

  let count = 0;
  for (let i = 0; i < input.length; i++) {
    const result = calculate2(input[i][0], input[i][1], input[i][1][0])
    count += result
  }

  console.log(count)
}

function main() {
  const file = readfile();

  const lines = file.split('\n');
  lines.pop();

  const input = lines.map(l => {
    const [answer, inputs] = l.split(': ');
    return [+answer, inputs.split(' ').map(e => +e)]
  })

  part1(input)
  part2(input)

}

main()
