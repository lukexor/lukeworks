import p5 from "p5";
import { awaitClickStart } from "../utils";

const fluidSimSketch = (p: p5): void => {
  const N = 90;
  const SCALE = 4;
  const ITER = 3;

  const DT = 0.03; // Time Step of Fluid
  const DIFFUSE = 0.00006; // Diffusion of Fluid
  const VISC = 0.0000000015; // Viscosity of Fluid
  const VEL = 2.8; // Velocity of fluid from perlin noise

  let fluid: Fluid;
  let demo = true;
  let fx = 0;
  let fy = 0;
  let px = 0;
  let py = 0;

  p.disableFriendlyErrors = true;

  p.setup = () => {
    const canvas = p.createCanvas(N * SCALE, N * SCALE);
    const cx = (p.windowWidth - p.width) / 2;
    const cy = (p.windowHeight - p.height) / 2;
    canvas.position(cx, cy);
    canvas.style("border", "2px solid #222");

    p.rectMode(p.CENTER);
    p.noStroke();

    fluid = new Fluid(DT, DIFFUSE, VISC);

    fx = p.width / 2;
    fy = p.height - 30;

    p.frameRate(30);
    awaitClickStart(p, undefined, () => {
      p.cursor(p.CROSS);
      p.mousePressed = () => {
        demo = false;
      };
      p.mouseReleased = () => {
        demo = true;
      };

      p.mouseMoved = () => {
        if (p.mouseIsPressed) {
          drag(p.mouseX, p.mouseY);
        }
      };

      p.touchMoved = () => {
        drag(p.mouseX, p.mouseY);
        return false;
      };

      p.touchStarted = () => {
        demo = false;
        return false;
      };
      p.touchEnded = () => {
        demo = true;
        return false;
      };
    });
  };

  p.draw = () => {
    if (!p.isLooping()) {
      return;
    }
    if (demo) {
      drag(fx, fy);
    } else if (p.mouseIsPressed) {
      drag(p.mouseX, p.mouseY);
    }

    fluid.step();
    fluid.draw();
  };

  const drag = (cx: number, cy: number) => {
    const x = cx / SCALE;
    const y = cy / SCALE;
    const amtX = cx - px;
    const amtY = cy - py;
    px = cx;
    py = cy;
    const vel = p.createVector(amtX, amtY);
    vel.setMag(VEL);
    for (let i = -2; i <= 2; ++i) {
      for (let j = -2; j <= 2; ++j) {
        fluid.addDensity(x + i, y + j, p.random(50, 100));
        fluid.addVelocity(x + i, y + j, vel.x, vel.y);
      }
    }
    return false;
  };

  const get = (data: number[], idx: number): number => {
    let result = data[idx];
    if (result === undefined) {
      console.log("out of bounds", data.length, idx);
      p.noLoop();
      result = 0;
    }
    return result;
  };

  class Fluid {
    size: number;
    dt: number;
    diff: number;
    visc: number;
    s: number[];
    density: number[];
    velx: number[];
    vely: number[];
    velx0: number[];
    vely0: number[];

    constructor(dt: number, diffusion: number, viscosity: number) {
      this.size = N;
      this.dt = dt;
      this.diff = diffusion;
      this.visc = viscosity;

      const count = N * N;
      this.s = new Array(count).fill(0);
      this.density = new Array(count).fill(0);

      this.velx = new Array(count).fill(0);
      this.vely = new Array(count).fill(0);

      this.velx0 = new Array(count).fill(0);
      this.vely0 = new Array(count).fill(0);
    }

    step() {
      diffuse(1, this.velx0, this.velx, this.visc, this.dt);
      diffuse(2, this.vely0, this.vely, this.visc, this.dt);

      project(this.velx0, this.vely0, this.velx, this.vely);

      advect(1, this.velx, this.velx0, this.velx0, this.vely0, this.dt);
      advect(2, this.vely, this.vely0, this.velx0, this.vely0, this.dt);

      project(this.velx, this.vely, this.velx0, this.vely0);

      diffuse(0, this.s, this.density, this.diff, this.dt);
      advect(0, this.density, this.s, this.velx, this.vely, this.dt);
    }

    addDensity(x: number, y: number, amount: number) {
      const idx = IDX(x, y);
      this.density[idx] += amount;
      const vel = p.createVector(
        p.random(-2 * VEL, 2 * VEL),
        p.random(-VEL / 2)
      );
      this.addVelocity(x, y, vel.x, vel.y);
    }

    addVelocity(x: number, y: number, amountX: number, amountY: number) {
      const idx = IDX(x, y);
      this.velx[idx] += amountX;
      this.vely[idx] += amountY;
    }

    draw() {
      for (let i = 1; i < N - 1; ++i) {
        for (let j = 1; j < N - 1; ++j) {
          const x = i * SCALE;
          const y = j * SCALE;
          const idx = IDX(i, j);

          // Draw density
          const d = get(this.density, idx);
          const m = d / 125;
          const f = m * d;
          p.fill(f, f / 3, 0, 150);
          p.square(x, y, SCALE);
        }
      }
    }
  }

  const set_bounds = (b: number, x: number[]) => {
    const nLen = N - 1;
    // Top and bottom rows
    for (let i = 1; i < nLen; ++i) {
      const top = get(x, IDX(i, 1));
      const bot = get(x, IDX(i, N - 2));
      x[IDX(i, 0)] = b == 2 ? -top : top;
      x[IDX(i, N - 1)] = b == 2 ? -bot : bot;
    }
    // Left and Right cols
    for (let j = 1; j < nLen; ++j) {
      const left = get(x, IDX(1, j));
      const right = get(x, IDX(N - 2, j));
      x[IDX(0, j)] = b == 1 ? -left : left;
      x[IDX(N - 1, j)] = b == 1 ? -right : right;
    }

    x[IDX(0, 0)] = 0.5 * get(x, IDX(1, 0)) + get(x, IDX(0, 1)); // top left
    x[IDX(0, N - 1)] = 0.5 * get(x, IDX(1, N - 2)) + get(x, IDX(0, N - 2)); // bottom left
    x[IDX(N - 1, 0)] = 0.5 * get(x, IDX(N - 2, 0)) + get(x, IDX(N - 1, 1)); // top right
    x[IDX(N - 1, N - 1)] =
      0.5 * get(x, IDX(N - 2, N - 2)) + get(x, IDX(N - 2, N - 2)); // bottom right
  };

  const linear_solve = (
    b: number,
    x: number[],
    x0: number[],
    a: number,
    c: number
  ) => {
    const cRecip = 1.0 / c;
    const nLen = N - 1;
    for (let t = 0; t < ITER; ++t) {
      for (let j = 1; j < nLen; ++j) {
        for (let i = 1; i < nLen; ++i) {
          const idx = IDX(i, j);
          x[idx] =
            (get(x0, idx) +
              a *
                (get(x, IDX(i + 1, j)) +
                  get(x, IDX(i - 1, j)) +
                  get(x, IDX(i, j + 1)) +
                  get(x, IDX(i, j - 1)))) *
            cRecip;
        }
      }
    }
    set_bounds(b, x);
  };

  const diffuse = (
    b: number,
    x: number[],
    x0: number[],
    diff: number,
    dt: number
  ) => {
    const a = dt * diff * (N - 2) * (N - 2);
    linear_solve(b, x, x0, a, 1 + 6 * a);
  };

  const project = (
    velx: number[],
    vely: number[],
    p: number[],
    div: number[]
  ) => {
    const nLen = N - 1;

    for (let j = 1; j < nLen; ++j) {
      for (let i = 1; i < nLen; ++i) {
        const idx = IDX(i, j);
        div[idx] =
          (-0.5 *
            (get(velx, IDX(i + 1, j)) -
              get(velx, IDX(i - 1, j)) +
              get(vely, IDX(i, j + 1)) -
              get(vely, IDX(i, j - 1)))) /
          N;
        p[idx] = 0;
      }
    }
    set_bounds(0, div);
    set_bounds(0, p);
    linear_solve(0, p, div, 1, 6);

    for (let j = 1; j < nLen; ++j) {
      for (let i = 1; i < nLen; ++i) {
        const idx = IDX(i, j);
        velx[idx] -= 0.5 * (get(p, IDX(i + 1, j)) - get(p, IDX(i - 1, j))) * N;
        vely[idx] -= 0.5 * (get(p, IDX(i, j + 1)) - get(p, IDX(i, j - 1))) * N;
      }
    }
    set_bounds(1, velx);
    set_bounds(2, vely);
  };

  const advect = (
    b: number,
    d: number[],
    d0: number[],
    velx: number[],
    vely: number[],
    dt: number
  ) => {
    let i0, i1, j0, j1;

    const dtx = dt * (N - 2);
    const dty = dt * (N - 2);

    let s0, s1, t0, t1;

    const nLen = N - 1;
    for (let j = 1; j < nLen; ++j) {
      for (let i = 1; i < nLen; ++i) {
        const idx = IDX(i, j);
        let x = i - dtx * get(velx, idx);
        let y = j - dty * get(vely, idx);

        if (x < 0.5) x = 0.5;
        if (x > N + 0.5) x = N + 0.5;
        i0 = p.floor(x);
        i1 = i0 + 1.0;
        if (y < 0.5) y = 0.5;
        if (y > N + 0.5) y = N + 0.5;
        j0 = p.floor(y);
        j1 = j0 + 1.0;

        s1 = x - i0;
        s0 = 1.0 - s1;
        t1 = y - j0;
        t0 = 1.0 - t1;

        const pd = get(d, idx);
        d[idx] =
          s0 * (t0 * get(d0, IDX(i0, j0)) + t1 * get(d0, IDX(i0, j1))) +
          s1 * (t0 * get(d0, IDX(i1, j0)) + t1 * get(d0, IDX(i1, j1)));
        d[idx] = p.constrain(get(d, idx), pd - 150, 350);
      }
    }
    set_bounds(b, d);
  };

  function IDX(x: number, y: number) {
    x = p.constrain(x, 0, N - 1);
    y = p.constrain(y, 0, N - 1);
    return p.floor(x) + p.floor(y) * N;
  }
};

export default fluidSimSketch;
