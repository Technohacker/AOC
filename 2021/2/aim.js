let lineReader = require('readline').createInterface({
    input: require('fs').createReadStream('file.in')
});

let coords = [0, 0];
let aim = 0;

lineReader.on('line', function (line) {
    let [command, n] = line.split(" ");
    n = parseInt(n);

    switch (command) {
        case "forward":
            coords[0] += n;
            coords[1] += n * aim;
            break;
        case "down":
            aim += n;
            break;
        case "up":
            aim -= n;
            break;
    }
});

lineReader.on("close", () => {
    console.log(coords[0] * coords[1]);
});