import p5 from "p5";
import { awaitClickStart } from "../utils";

const fireworksSketch = (p: p5): void => {
  const fireworks: Firework[] = [];
  const stars: Star[] = [];
  const starCount = 100;
  let gravity: p5.Vector;

  p.disableFriendlyErrors = true;

  p.setup = () => {
    p.createCanvas(p.windowWidth, p.windowHeight);
    p.background(0);
    for (let i = 0; i < starCount; ++i) {
      stars.push(new Star());
    }
    gravity = p.createVector(0, 0.2);

    awaitClickStart(p, undefined, () => {
      p.mousePressed = () => {
        p.cursor(p.CROSS);
        fireworks.push(new Firework(p.mouseX, p.mouseY));
      };
    });
  };

  p.draw = () => {
    if (!p.isLooping()) {
      return;
    }

    p.background(0, 25);
    if (p.random(1) < 0.05) {
      fireworks.push(new Firework());
    }
    stars.forEach((s) => s.draw());
    fireworks.reverse().forEach((firework, i) => {
      if (firework.done()) {
        fireworks.splice(i, 1);
      }
      firework.update();
      firework.draw();
    });
  };

  class Star {
    alpha: number;
    size: number;
    pos: p5.Vector;

    constructor() {
      this.alpha = p.random(5, 30);
      this.size = p.random(4);
      this.pos = p.createVector(p.random(p.width), p.random(p.height));
    }

    draw() {
      p.stroke(255, this.alpha);
      p.strokeWeight(this.size);
      p.point(this.pos);
    }
  }

  class Firework {
    color: p5.Color;
    particles: Particle[];
    particleCount: number;
    explodeDelay: number;
    exploded: boolean;

    constructor(x?: number, y?: number) {
      this.color = p.color(p.random(255), p.random(255), p.random(255));
      const firework = new Particle(p.random(p.width), p.height, this.color);
      firework.vel = p.createVector(0, p.random(-13, -8));
      this.particles = [firework];
      this.particleCount = p.random(50, 200);
      this.exploded = false;
      this.explodeDelay = p.random(0, 6);

      if (x && y) {
        firework.pos.x = x;
        firework.pos.y = y;
        this.explode();
      }
    }

    update() {
      this.particles.reverse().forEach((particle, i) => {
        particle.applyForce(gravity);
        particle.update();
        if (this.exploded) {
          particle.vel.mult(0.9);
          particle.lifespan -= 3;
          if (particle.done()) {
            this.particles.splice(i, 1);
          }
        } else if (particle.vel.y >= 0) {
          this.explode();
        }
      });
    }

    done() {
      if (this.exploded && this.particles.length === 0) {
        return true;
      }
      return false;
    }

    explode() {
      this.exploded = true;
      const firework = this.particles.pop();
      if (firework) {
        for (let i = 0; i < this.particleCount; ++i) {
          const particle = new Particle(
            firework.pos.x,
            firework.pos.y,
            this.color
          );
          particle.vel = p5.Vector.random2D();
          particle.vel.mult(p.random(2, 10));
          this.particles.push(particle);
        }
      }
    }

    draw() {
      if (this.exploded) {
        p.strokeWeight(2);
      } else {
        p.strokeWeight(4);
      }
      this.particles.forEach((p) => p.draw());
    }
  }

  class Particle {
    pos: p5.Vector;
    vel: p5.Vector;
    acc: p5.Vector;
    color: p5.Color;
    lifespan: number;

    constructor(x: number, y: number, color: p5.Color) {
      this.pos = p.createVector(x, y);
      this.vel = p.createVector(0, 0);
      this.acc = p.createVector(0, 0);
      this.color = color;
      this.lifespan = 255;
    }

    update() {
      this.vel.add(this.acc);
      this.pos.add(this.vel);
      this.acc.mult(0);
    }

    done() {
      if (this.lifespan <= 0) {
        return true;
      }
      return false;
    }

    applyForce(force: p5.Vector) {
      this.acc.add(force);
    }

    draw() {
      if (this.lifespan > 0) {
        p.stroke(
          p.red(this.color),
          p.green(this.color),
          p.blue(this.color),
          this.lifespan
        );
        p.point(this.pos);
      }
    }
  }
};

export default fireworksSketch;
