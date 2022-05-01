import p5 from "p5";
import { awaitClickStart } from "../utils";

const raycasting2DSketch = (p: p5) => {
  const boundaries: Boundary[] = [];
  const boundaryCount = 5;
  let light: Light;
  let mouseMove = false;
  let xOffset = 0;
  let yOffset = 10000;

  p.disableFriendlyErrors = true;

  p.setup = () => {
    p.createCanvas(p.windowWidth, p.windowHeight);
    boundaries.push(new Boundary(0, 0, p.width, 0)); // Top
    boundaries.push(new Boundary(p.width, 0, p.width, p.height)); // Right
    boundaries.push(new Boundary(0, p.height, p.width, p.height)); // Bottom
    boundaries.push(new Boundary(0, 0, 0, p.height)); // Left
    for (let i = 0; i < boundaryCount; ++i) {
      const x1 = p.random(p.width);
      const y1 = p.random(p.height);
      const x2 = p.random(p.width);
      const y2 = p.random(p.height);
      boundaries.push(new Boundary(x1, y1, x2, y2));
    }
    light = new Light();

    awaitClickStart(p, undefined, () => {
      p.mousePressed = () => {
        mouseMove = !mouseMove;
      };
    });
  };

  p.draw = () => {
    if (!p.isLooping()) {
      return;
    }
    p.background(51);
    light.update(boundaries);
    boundaries.forEach((b) => b.draw());
    light.draw();
  };

  class Light {
    pos: p5.Vector;
    rays: Ray[];

    constructor() {
      this.pos = p.createVector(p.width / 2, p.height / 2);
      this.rays = [];
      for (let angle = 0; angle < 360; angle += 1) {
        this.rays.push(new Ray(this.pos, p.radians(angle)));
      }
    }

    draw() {
      p.fill(255);
      this.rays.forEach((r) => r.draw());
    }

    update(boundaries: Boundary[]) {
      if (mouseMove) {
        light.pos.x = p.mouseX;
        light.pos.y = p.mouseY;
      } else {
        light.pos.x = p.noise(xOffset) * p.width;
        light.pos.y = p.noise(yOffset) * p.height;
        xOffset += 0.01;
        yOffset += 0.01;
      }
      this.rays.forEach((r) => {
        let closest;
        let closestDist = Infinity;
        boundaries.forEach((b) => {
          const pt = r.cast(b);
          if (pt) {
            const dist = window.p5.Vector.dist(this.pos, pt);
            if (dist < closestDist) {
              closestDist = dist;
              closest = pt;
            }
          }
        });
        if (closest) {
          r.lookAt(closest);
        }
      });
    }
  }

  class Ray {
    pos: p5.Vector;
    looking: p5.Vector;

    constructor(pos: p5.Vector, angle: number) {
      this.pos = pos;
      this.looking = window.p5.Vector.fromAngle(angle);
    }

    draw() {
      p.stroke(255, 100);
      p.strokeWeight(1);
      p.push();
      p.translate(this.pos.x, this.pos.y);
      p.line(0, 0, this.looking.x, this.looking.y);
      p.pop();
    }

    lookAt(pt: p5.Vector) {
      this.looking.x = pt.x - this.pos.x;
      this.looking.y = pt.y - this.pos.y;
    }

    cast(boundary: Boundary) {
      // Formula: https://en.wikipedia.org/wiki/Line%E2%80%93p.line_intersection
      const x1 = boundary.start.x;
      const y1 = boundary.start.y;
      const x2 = boundary.end.x;
      const y2 = boundary.end.y;

      const x3 = this.pos.x;
      const y3 = this.pos.y;
      const x4 = this.pos.x + this.looking.x;
      const y4 = this.pos.y + this.looking.y;

      const denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
      if (denominator == 0) {
        return;
      }
      const t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denominator;
      const u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / denominator;

      if (t > 0 && t < 1 && u > 0) {
        return p.createVector(x1 + t * (x2 - x1), y1 + t * (y2 - y1));
      }
      return;
    }
  }

  class Boundary {
    start: p5.Vector;
    end: p5.Vector;

    constructor(x1: number, y1: number, x2: number, y2: number) {
      this.start = p.createVector(x1, y1);
      this.end = p.createVector(x2, y2);
    }

    draw() {
      p.stroke(255);
      p.strokeWeight(2);
      p.line(this.start.x, this.start.y, this.end.x, this.end.y);
    }
  }
};

export default raycasting2DSketch;
