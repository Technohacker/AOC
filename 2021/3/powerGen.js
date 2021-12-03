let lineReader = require('readline').createInterface({
    input: require('fs').createReadStream('file.in')
});

let arr = []

lineReader.on('line', function (line) {
    arr.push(line.split("").map(x => parseInt(x, 10)));
});

lineReader.on("close", () => {
    let numOnes = []
    for (let _ of arr[0]) {
        numOnes.push(0);
    }

    for (let num of arr) {
        for (let i = 0; i < numOnes.length; i += 1) {
            if (num[i] === 1) {
                numOnes[i] += 1;
            }
        }
    }

    let gamma = [];
    let epsilon = [];
    for (let i = 0; i < numOnes.length; i += 1) {
        if (numOnes[i] > (arr.length / 2)) {
            gamma[i] = 1;
            epsilon[i] = 0;
        } else {
            gamma[i] = 0;
            epsilon[i] = 1;
        }
    }

    gamma = parseInt(gamma.join(""), 2);
    epsilon = parseInt(epsilon.join(""), 2);

    console.log(gamma * epsilon);
});