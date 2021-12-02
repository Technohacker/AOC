let lineReader = require('readline').createInterface({
    input: require('fs').createReadStream('file.in')
});

let arr = []

lineReader.on('line', function (line) {
    arr.push(parseInt(line, 10));
});

lineReader.on("close", () => {
    let count = 0;

    let prevMeasure = arr[0];
    for (let measure of arr) {
        if (measure > prevMeasure) {
            count += 1;
        }
        prevMeasure = measure;
    }

    console.log(count);
});