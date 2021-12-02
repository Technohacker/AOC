let lineReader = require('readline').createInterface({
    input: require('fs').createReadStream('file.in')
});

let arr = []

lineReader.on('line', function (line) {
    arr.push(parseInt(line, 10));
});

lineReader.on("close", () => {
    let count = 0;

    let prevWindow = arr[0] + arr[1] + arr[2];
    let windowSum = prevWindow;
    for (let i = 3; i < arr.length; i += 1) {
        windowSum -= arr[i - 3];
        windowSum += arr[i];

        if (windowSum > prevWindow) {
            count += 1;
        }
        prevWindow = windowSum;
    }

    console.log(count);
});