let lineReader = require('readline').createInterface({
    input: require('fs').createReadStream('file.in')
});

let oxy = []
let co2 = []

lineReader.on('line', function (line) {
    let arr = line.split("").map(x => parseInt(x, 10));
    oxy.push(arr);
    co2.push(arr);
});

lineReader.on("close", () => {
    for (let i = 0; i < oxy[0].length; i += 1) {
        let numOnesOxy = oxy.map(x => x[i]).filter(x => x === 1).length;
        let numOnesCo2 = co2.map(x => x[i]).filter(x => x === 1).length;

        let mostCommonOxy = null;
        let leastCommonCo2 = null;

        if (numOnesOxy >= (oxy.length / 2)) {
            mostCommonOxy = 1;
        } else {
            mostCommonOxy = 0;
        }

        if (numOnesCo2 < (co2.length / 2)) {
            leastCommonCo2 = 1;
        } else {
            leastCommonCo2 = 0;
        }

        if (oxy.length > 1) {
            oxy = oxy.filter(x => {
                return x[i] === mostCommonOxy;
            });
        }

        if (co2.length > 1) {
            co2 = co2.filter(x => {
                return x[i] === leastCommonCo2;
            });
        }
    }

    oxy = parseInt(oxy[0].join(""), 2);
    co2 = parseInt(co2[0].join(""), 2);

    console.log(oxy * co2);
});