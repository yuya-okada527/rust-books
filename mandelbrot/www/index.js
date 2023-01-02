"use strict";

function draw(ctx, canvas_w, canvas_h, data) {
  let img = new ImageData(new Uint8ClampedArray(data), canvas_w, canvas_h);
  ctx.putImageData(img, 0, 0);
}

const X_MIN = -1.5;
const X_MAX = 0.5;
const Y_MIN = -1.0;
const Y_MAX = 1.0;
const MAX_ITER = 64;

console.log("start loading wask");
const mandelbrot = import("../pkg").catch(console.error);

Promise.all([mandelbrot]).then(async function ([
  { generate_mandelbrot_set, draw_mandelbrot_set },
]) {
  console.log("finished loading wasm");
  const renderBtn = document.getElementById("render");
  renderBtn.addEventListener("click", () => {
    draw_mandelbrot_set();
    let wasmResult = null;
    {
      const CANVAS_ID = "canvas_hybrid";
      let canvas = document.getElementById(CANVAS_ID);
      let context = canvas.getCotext("2d");
      const canvasWidth = canvas.width;
      const canvasHeight = canvas.height;
      const generateStartTime = Date.now();
      wasmResult = generate_mandelbrot_set(
        canvasWidth,
        canvasHeight,
        X_MIN,
        X_MAX,
        Y_MIN,
        Y_MAX,
        MAX_ITER
      );
      const generateEndTime = Date.now();
      const drawStartTime = Date.now();
      draw(context, canvasWidth, canvasHeight, wasmResult);
      const drawEndTime = Date.now();
      const generateElapsed = generateEndTime - generateStartTime;
      const drawElapsed = drawEndTime - drawStartTime;
      console.log(`\tgenerate: wasm\tgenerate_elapsed: ${generateElapsed}[ms]`);
      console.log(`\tdraw: js\tdraw_elapsed: ${drawElapsed}[ms]`);
    }
  });
});
