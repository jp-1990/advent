const { readfile } = require("../utils");

/**
 * @param input{string}
 */
function part1(input) {
  // digits alternate between file and free space
  // each file digit has an incrementing index

  let disk = [];

  let id = 0;
  let file = 1;
  let free = 0;

  for (let i = 0; i < input.length; i++) {
    const n = +input[i];
    for (let x = 0; x < n; x++) {
      if (file) {
        disk.push(`${id}`);
      } else {
        disk.push(".");
        free++;
      }
    }

    if (file) {
      file = 0;
      id++;
    } else {
      file = 1;
    }
  }

  let defragged = [];
  let checksum = 0;
  let left = 0;
  let right = disk.length - 1;

  while (right >= left && left < disk.length - free) {
    const l = disk[left];
    left++;
    if (l !== ".") {
      defragged.push(+l);
      checksum += +l * (defragged.length - 1);
      continue;
    }

    let r = disk[right];
    while (r === ".") {
      right--;
      r = disk[right];
    }

    // r must be file at this point
    // move the r
    defragged.push(+r);
    checksum += +r * (defragged.length - 1);
    right--;
  }

  console.log(checksum);
}

/**
 * @param input{string}
 */
function part2(input) {
  let disk = [];

  let id = 0;
  let file = 1;
  let freeHash = {};
  let fileHash = {};

  for (let i = 0; i < input.length; i++) {
    const n = +input[i];
    const startIndex = disk.length;
    for (let x = 0; x < n; x++) {
      if (file) {
        if (x === 0) fileHash[startIndex] = 0;
        fileHash[startIndex]++;
        disk.push(`${id}`);
      } else {
        if (x === 0) freeHash[startIndex] = 0;
        freeHash[startIndex]++;
        disk.push(".");
      }
    }

    if (file) {
      file = 0;
      id++;
    } else {
      file = 1;
    }
  }

  let checksum = 0;

  const fileIndexes = Object.keys(fileHash);
  const totalFiles = fileIndexes.length;
  // loop files backwards by starting index in disk
  for (let i = totalFiles - 1; i >= 0; i--) {
    let fileLocation = +fileIndexes[i];
    const fileId = i;
    const fileSize = fileHash[fileLocation];

    // loop free space forwards by starting index in disk
    // if file cant be moved leave it alone
    const spaceIndexes = Object.keys(freeHash);
    inner: for (let j = 0; j < spaceIndexes.length; j++) {
      const index = spaceIndexes[j];
      const space = freeHash[index];

      if (space >= fileSize && index < fileLocation) {
        // move file to space
        freeHash[fileLocation] = fileSize;

        // group space
        let newSpaceIndex = fileLocation;
        let newSpaceSize = fileSize;
        // if prev space index + prev space size = fileLocation then we have contiguous space
        // if new space index + new space size = next space index then we have contiguous space
        const currentSpaceIndexes = Object.keys(freeHash);
        const targetIndex = currentSpaceIndexes.findIndex(
          (e) => e == fileLocation,
        );

        const prevIndex = currentSpaceIndexes[targetIndex - 1];
        const prevSpace = freeHash[prevIndex];
        if (prevSpace && +prevIndex + prevSpace == fileLocation) {
          delete freeHash[prevIndex];
          newSpaceIndex -= prevSpace;
          newSpaceSize += prevSpace;
        }

        const nextIndex = currentSpaceIndexes[targetIndex + 1];
        const nextSpace = freeHash[nextIndex];
        if (nextSpace && newSpaceIndex + newSpaceSize == nextIndex) {
          delete freeHash[nextIndex];
          newSpaceSize += nextSpace;
        }

        delete freeHash[fileLocation];
        freeHash[newSpaceIndex] = newSpaceSize;
        fileLocation = +index;

        // update free space hash
        const remainingSpace = space - fileSize;
        const newIndex = +index + fileSize;

        delete freeHash[index];
        if (remainingSpace) {
          freeHash[newIndex] = remainingSpace;
        }

        break inner;
      }
    }

    // update checksum with location of file
    for (let j = 0; j < fileSize; j++) {
      checksum += (fileLocation + j) * fileId;
    }
  }

  console.log(checksum);
}

function main() {
  console.log("start");

  const input = readfile();

  part1(input.trim());
  part2(input.trim());

  console.log("end");
}

main();
