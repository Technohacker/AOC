let lineReader = require('readline').createInterface({
    input: require('fs').createReadStream('file.in')
});

let coords = [0, 0];

lineReader.on('line', function (line) {
    let [command, n] = line.split(" ");
    n = parseInt(n);

    switch (command) {
        case "forward":
            coords[0] += n;
            break;
        case "down":
            coords[1] += n;
            break;
        case "up":
            coords[1] -= n;
            break;
    }
});

lineReader.on("close", () => {
    console.log(coords[0] * coords[1]);
});