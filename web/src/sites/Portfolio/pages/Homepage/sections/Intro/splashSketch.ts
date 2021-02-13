import p5 from "p5";
import theme from "sites/Portfolio/theme";

const splashSketch = (p: p5): void => {
  p.disableFriendlyErrors = true;

  const headerHeight = (document.querySelector("section#splash") as HTMLElement)
    .offsetTop;

  const electrons: Electron[] = [];
  const maxElectrons = 10;
  const spawnRate = 0.01;

  p.setup = () => {
    p.createCanvas(p.windowWidth, p.windowHeight - headerHeight);
    p.angleMode(p.DEGREES);
  };

  p.windowResized = () => {
    p.resizeCanvas(p.windowWidth, p.windowHeight - headerHeight);
  };

  p.draw = () => {
    p.clear();
    if (electrons.length < maxElectrons && p.random(1) < spawnRate) {
      electrons.push(new Electron());
    }
    for (let i = electrons.length - 1; i >= 0; --i) {
      const e = electrons[i];
      if (e.offscreen()) {
        electrons.splice(i, 1);
      }
      e.draw();
    }
  };

  class Electron {
    numSegments: number;
    perSegment: number;
    speed: number;
    turned: boolean;
    turnRate: number;
    color: string;
    trailColor: string;
    xCor: number[];
    yCor: number[];
    radius: number;
    direction: string;

    constructor() {
      const right = p.windowWidth;
      const bottom = p.windowHeight;
      const randX = p.random(0, right);
      const randY = p.random(0, bottom);

      this.numSegments = 30;
      this.perSegment = 10;
      this.speed = 2;

      this.turned = false;
      this.turnRate = 0.1;

      this.color = theme.colors.primary;
      this.trailColor =
        p.random(1) < 0.5 ? theme.colors.accentDark : theme.colors.secondary;
      this.radius = 2;

      this.xCor = [];
      this.yCor = [];

      this.direction = "";

      let xStart = 0;
      let yStart = 0;
      switch (p.ceil(p.random(4))) {
        case 1:
          this.direction = "up";
          xStart = randX;
          yStart = bottom;
          break;
        case 2:
          this.direction = "down";
          xStart = randX;
          break;
        case 3:
          this.direction = "left";
          xStart = right;
          yStart = randY;
          break;
        case 4:
          this.direction = "right";
          yStart = randY;
          break;
        default:
      }

      for (let i = 0; i < this.numSegments; i++) {
        this.xCor.push(xStart);
        this.yCor.push(yStart);
      }
    }

    offscreen() {
      const w = p.windowWidth;
      const h = p.windowHeight;
      return (
        this.xCor[0] < 0 ||
        this.xCor[0] > w ||
        this.yCor[0] < 0 ||
        this.yCor[0] > h
      );
    }

    update() {
      for (let i = 0; i < this.numSegments - 1; i++) {
        this.xCor[i] = this.xCor[i + 1];
        this.yCor[i] = this.yCor[i + 1];
      }

      switch (this.direction) {
        case "up":
          this.xCor[this.numSegments - 1] = this.xCor[this.numSegments - 2];
          this.yCor[this.numSegments - 1] =
            this.yCor[this.numSegments - 2] - this.perSegment;
          break;
        case "down":
          this.xCor[this.numSegments - 1] = this.xCor[this.numSegments - 2];
          this.yCor[this.numSegments - 1] =
            this.yCor[this.numSegments - 2] + this.perSegment;
          break;
        case "left":
          this.xCor[this.numSegments - 1] =
            this.xCor[this.numSegments - 2] - this.perSegment;
          this.yCor[this.numSegments - 1] = this.yCor[this.numSegments - 2];
          break;
        case "right":
          this.xCor[this.numSegments - 1] =
            this.xCor[this.numSegments - 2] + this.perSegment;
          this.yCor[this.numSegments - 1] = this.yCor[this.numSegments - 2];
          break;
        default:
      }

      if (!this.turned && p.random(1) < this.turnRate) {
        this.turned = true;
        const coinFlip = p.random(1) < 0.5;
        switch (this.direction) {
          case "up":
            this.direction = coinFlip ? "left" : "right";
            break;
          case "down":
            this.direction = coinFlip ? "right" : "left";
            break;
          case "left":
            this.direction = coinFlip ? "up" : "down";
            break;
          case "right":
            this.direction = coinFlip ? "down" : "up";
            break;
          default:
        }
      }
    }

    draw() {
      p.stroke(this.trailColor);
      p.strokeWeight(this.radius);
      for (let i = 0; i < this.numSegments - 1; i++) {
        p.line(this.xCor[i], this.yCor[i], this.xCor[i + 1], this.yCor[i + 1]);
      }

      p.stroke(this.color);
      p.strokeWeight(this.radius + 1);
      p.point(this.xCor[this.xCor.length - 1], this.yCor[this.yCor.length - 1]);

      for (let i = 0; i < this.speed; ++i) {
        this.update();
      }
    }
  }
};

export default splashSketch;
