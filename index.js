// reference: https://github.com/tsoding/rust-browser-game

async function start() {
    const game = await WebAssembly.instantiateStreaming(fetch("main.wasm"));

    const memory = new Uint8Array(game.instance.exports.memory.buffer);
    game.instance.exports.init();

    const displayAddr = game.instance.exports.get_display();
    console.log("Memory address of display:", displayAddr);
    console.log("Memory:", memory.length);

    const width = game.instance.exports.get_width();
    const height = game.instance.exports.get_height();
    console.log("Game size from WASM", width, height);
    const size = width * height;

    const canvas = document.getElementById("canvas");
    const ctx = canvas.getContext("2d");

    let curFrame = 0;

    while (true) {
        // draw a frame
        game.instance.exports.step(0.1, curFrame);
        const pixels = new Uint8ClampedArray(
            memory.subarray(displayAddr, displayAddr + 4 * size)
        );
        const frame = new ImageData(pixels, width, height);
        ctx.putImageData(frame, 0, 0);
        curFrame += 1;
        await new Promise(r => setTimeout(r, 10));
    }
}
start().catch((e) => console.error(e));
