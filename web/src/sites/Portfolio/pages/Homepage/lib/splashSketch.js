import theme from "sites/Portfolio/theme";

const splashSketch = (p5) => {
  p5.disableFriendlyErrors = true;

  const electrons = [];
  const maxElectrons = 10;
  const spawnRate = 0.02;

  p5.setup = () => {
    p5.createCanvas(p5.windowWidth, p5.windowHeight);
    p5.angleMode(p5.DEGREES);
  };

  p5.draw = () => {
    p5.clear();
    if (electrons.length < maxElectrons && p5.random(1) < spawnRate) {
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
    constructor() {
      const right = p5.windowWidth;
      const bottom = p5.windowHeight;
      const randX = p5.random(0, right);
      const randY = p5.random(0, bottom);

      this.numSegments = 30;
      this.perSegment = 10;
      this.speed = 2;

      this.turned = false;
      this.turnRate = 0.1;

      this.color = theme.colors.accentLight;
      this.trailColor =
        p5.random(1) < 0.5 ? theme.colors.accentDark : theme.colors.secondary;
      this.radius = 2;

      this.xCor = [];
      this.yCor = [];

      let xStart = 0;
      let yStart = 0;
      switch (p5.ceil(p5.random(4))) {
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
      const w = p5.windowWidth;
      const h = p5.windowHeight;
      return (
        this.xCor[this.xCor.length - 1] < 0 ||
        this.xCor[this.xCor.length - 1] > w ||
        this.yCor[this.yCor.length - 1] < 0 ||
        this.yCor[this.yCor.length - 1] > h
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

      if (!this.turned && p5.random(1) < this.turnRate) {
        this.turned = true;
        const coinFlip = p5.random(1) < 0.5;
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
      p5.stroke(this.trailColor);
      p5.strokeWeight(this.radius);
      for (let i = 0; i < this.numSegments - 1; i++) {
        p5.line(this.xCor[i], this.yCor[i], this.xCor[i + 1], this.yCor[i + 1]);
      }

      p5.stroke(this.color);
      p5.strokeWeight(this.radius + 1);
      p5.point(
        this.xCor[this.xCor.length - 1],
        this.yCor[this.yCor.length - 1]
      );

      for (let i = 0; i < this.speed; ++i) {
        this.update();
      }
    }
  }
};

export default splashSketch;
