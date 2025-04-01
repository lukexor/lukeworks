import p5 from "p5";
import { awaitClickStart } from "./utils";

export default function fourierSketch(p: p5) {
  type Fourier = {
    re: number;
    im: number;
    freq: number;
    amp: number;
    phase: number;
  };

  const MODES = Object.freeze({
    COLLECT: "COLLECT",
    DEMO: "DEMO",
    DFT: "DFT",
  });
  let mode: string = MODES.DEMO;
  let border: number;
  let drawing: Drawing;
  let waveSlider: p5.Element;

  p.disableFriendlyErrors = true;

  p.preload = () => {};
  p.setup = () => {
    p.createCanvas(p.windowWidth, p.windowHeight - 30);

    awaitClickStart(p, undefined, () => {
      border = p.width / 2;
      mode = MODES.DEMO;
      drawing = new Drawing();
      const label = p.createSpan("Iterations: ");
      waveSlider = p.createSlider(1, 10, 5);
      label.position(10, p.windowHeight - 25);
      waveSlider.position(90, p.windowHeight - 25);

      p.cursor(p.CROSS);
      setTimeout(() => {
        p.mousePressed = () => drawing.mousePressed();
        p.mouseReleased = () => drawing.mouseReleased();
      }, 200);
    });
  };

  p.draw = () => {
    if (!p.isLooping()) {
      return;
    }

    p.background(0);

    // Draw border
    p.stroke(255);
    p.line(border, 0, border, p.height);
    p.noStroke();
    p.fill(10);
    p.rect(0, 0, border, p.height);

    switch (mode) {
      case MODES.COLLECT:
        drawing.collect();
        break;
      case MODES.DEMO:
        drawing.demo();
        break;
      case MODES.DFT: {
        drawing.update();
        drawing.draw();
        break;
      }
    }
  };

  p.keyPressed = () => {
    if (p.keyCode === p.ESCAPE) {
      waveSlider.show();
      mode = MODES.DEMO;
    }
  };

  class Complex {
    re: number;
    im: number;

    constructor(a: number, b: number) {
      this.re = a;
      this.im = b;
    }

    addEq(c: Complex) {
      this.re += c.re;
      this.im += c.im;
    }

    mult(c: Complex) {
      // (a + bi)(c + di)
      // (ac - bd) + (ad + bc)i
      return new Complex(
        this.re * c.re - this.im * c.im,
        this.re * c.im + this.im * c.re,
      );
    }

    avg(N: number) {
      this.re /= N;
      this.im /= N;
    }

    get amp() {
      return p.sqrt(this.re * this.re + this.im * this.im);
    }

    get phase() {
      return p.atan2(this.im, this.re);
    }
  }

  class Drawing {
    time: number;
    data: Complex[];
    fourier: Fourier[];
    path: p5.Vector[];

    constructor() {
      this.time = 0;
      this.data = [];
      this.fourier = [];
      this.path = [];
    }

    clear() {
      this.data.length = 0;
      this.fourier.length = 0;
    }

    reset() {
      this.time = 0;
      this.path.length = 0;
    }

    mousePressed() {
      if (p.mouseY < p.height && p.mouseX < p.width && p.mouseX > p.width / 2) {
        mode = MODES.COLLECT;
        waveSlider.hide();
        drawing.clear();
        drawing.reset();
        return false;
      }
      return true;
    }

    mouseReleased() {
      if (
        p.mouseY < p.height &&
        p.mouseX < p.width &&
        p.mouseX > p.width / 2 &&
        this.data.length > 0
      ) {
        mode = MODES.DFT;
        drawing.dft();
        return false;
      }
      return true;
    }

    collect() {
      const offsetX = p.width / 2 + p.width / 4;
      const offsetY = p.height / 2;
      if (p.mouseX > p.width / 2) {
        this.data.push(new Complex(p.mouseX - offsetX, p.mouseY - offsetY));
      }
      p.stroke(255);
      p.noFill();
      p.beginShape();
      this.data.forEach((c) => p.vertex(c.re + offsetX, c.im + offsetY));
      p.endShape();
    }

    update() {
      this.time += p.TWO_PI / this.fourier.length;
      if (this.time > p.TWO_PI) {
        this.reset();
      }
    }

    draw() {
      const v = this.epicycles();
      this.path.unshift(v);

      p.translate(p.width / 2, 0);
      const start = this.path[0];
      if (start) {
        p.stroke("yellow");
        p.line(v.x - p.width / 2, v.y, start.x, start.y);
      }
      p.stroke("white");
      p.beginShape();
      p.noFill();
      this.path.forEach((c) => {
        p.vertex(c.x, c.y);
      });
      p.endShape();
    }

    epicycles() {
      // Position of epicycle p.circle center
      let x = p.width / 4;
      let y = p.height / 2;
      const rot = 0;
      this.fourier.forEach((f) => {
        const prevX = x;
        const prevY = y;
        x += f.amp * p.cos(f.freq * this.time + f.phase + rot);
        y += f.amp * p.sin(f.freq * this.time + f.phase + rot);

        p.stroke(255, 100);
        p.noFill();
        p.circle(prevX, prevY, 2 * f.amp);
        p.stroke(255);
        p.line(prevX, prevY, x, y);
        p.circle(x, y, 3);
      });
      return p.createVector(x, y);
    }

    demo() {
      let x = p.width / 4;
      let y = p.height / 2;
      for (let i = 0; i < waveSlider.value(); i++) {
        const prevX = x;
        const prevY = y;

        const n = i * 2 + 1;
        const radius = 60 * (4 / (n * p.PI));
        x += radius * p.cos(n * this.time);
        y += radius * p.sin(n * this.time);

        p.stroke(255, 100);
        p.noFill();
        p.circle(prevX, prevY, 2 * radius);

        p.stroke(255);
        p.line(prevX, prevY, x, y);
        p.circle(x, y, 3);
      }
      this.path.unshift(p.createVector(0, y));

      p.translate(p.width / 2, 0);

      const offsetX = 100;
      const start = this.path[0];
      if (start) {
        p.stroke("yellow");
        p.line(x - p.width / 2, y, offsetX, start.y);
      }
      p.stroke("white");
      p.beginShape();
      p.noFill();
      this.path.forEach((point, i) => {
        p.vertex(i + offsetX, point.y);
      });
      p.endShape();

      this.time += 0.05;

      if (this.path.length > 500) {
        this.path.pop();
      }
    }

    dft() {
      if (this.data.length === 0) {
        return;
      }
      const N = this.data.length;
      this.fourier.length = N;
      for (let k = 0; k < N; ++k) {
        const sum = new Complex(0, 0);
        for (let n = 0; n < N; ++n) {
          // X[k] = xys[n] * [cos(2*pi*k*n / N) - i * sin(2*pi*k*n / N)]
          const phi = (p.TWO_PI * k * n) / N;
          const data = this.data[n];
          if (data) {
            sum.addEq(data.mult(new Complex(p.cos(phi), -p.sin(phi))));
          }
        }
        sum.avg(N);
        const re = sum.re;
        const im = sum.im;
        const freq = k;
        const amp = sum.amp;
        const phase = sum.phase;
        this.fourier[k] = { re, im, freq, amp, phase };
      }
      this.fourier.sort((a, b) => b.amp - a.amp);
    }
  }
}
