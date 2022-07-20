import p5 from "p5";
import { awaitClickStart } from "./utils";

export default function asteroidSketch(p: p5): void {
  const SPACE = 32;
  const R = 82;

  let game: Game;
  let touching = false;
  let touchStart = 0;
  let textX = 0;
  const textY = 150;

  p.disableFriendlyErrors = true;

  p.preload = () => {};
  p.setup = () => {
    p.createCanvas(p.windowWidth, p.windowHeight);

    textX = p.width / 2;
    game = new Game();
    p.textFont("Courier");
    p.fill(255);

    awaitClickStart(
      p,
      () => {
        p.textAlign(p.CENTER);
        p.textSize(50);
        p.text("ASTEROIDS", textX, textY);
        p.textSize(20);
        p.text("CLICK OR TAP TO PLAY", textX, textY + 40);
        p.textSize(15);
        p.text(
          "INSTRUCTIONS:\n\n" +
            "LEFT / RIGHT: STEER SHIP\n" +
            "UP: ACCELERATE\n" +
            "SPACE: FIRE\n" +
            "ESCAPE: TOGGLE PAUSE\n" +
            "R: START NEW GAME\n",
          textX,
          textY + 80,
        );
      },
      () => {
        p.touchStarted = () => {
          touching = true;
          touchStart = Date.now();
          return false;
        };
        p.touchEnded = () => {
          touching = false;
          if (Date.now() - touchStart < 200) {
            game.fire();
          }
          return false;
        };
      },
    );
  };

  p.draw = () => {
    if (!p.isLooping()) {
      return;
    }
    if (game.gameover) {
      p.background(0);
      p.textAlign(p.CENTER);
      p.textSize(50);
      p.text("GAME OVER", textX, textY);
      p.textSize(20);
      p.text("CLICK OR TAP TO RESTART", textX, textY + 40);
      p.mousePressed = () => game.start();
      p.noLoop();
    } else if (game.started) {
      p.background(0);
      game.update();
      game.draw();
    } else if (!game.started) {
      game.start();
    }
  };

  p.keyPressed = () => {
    game.keyPressed(p.keyCode);
    return false;
  };
  p.keyTyped = () => {
    return false;
  };
  p.keyReleased = () => {
    game.keyReleased(p.keyCode);
    return false;
  };

  class Game {
    ship: Ship;
    asteroids: Asteroid[];
    bullets: Bullet[];
    level: number;
    lives: number;
    score: number;
    paused: boolean;
    started: boolean;
    gameover: boolean;

    constructor() {
      this.ship = new Ship();
      this.asteroids = [];
      this.bullets = [];
      this.level = 1;
      this.lives = 4;
      this.score = 0;
      this.paused = false;
      this.started = false;
      this.gameover = false;
      this.spawnShip();
    }

    update() {
      this.ship.update();
      this.updateBullets();
      this.updateAsteroids();
      this.handleInputs();

      if (this.asteroids.length === 0) {
        this.level += 1;
        this.score += 1000;
        this.bullets.length = 0;
        for (let i = 0; i < this.level + 2; ++i) {
          this.asteroids.push(new Asteroid(this.ship));
        }
      }
    }

    handleInputs() {
      if (p.keyIsDown(p.LEFT_ARROW)) {
        this.ship.angle = (this.ship.angle - 0.1) % 360;
      } else if (p.keyIsDown(p.RIGHT_ARROW)) {
        this.ship.angle = (this.ship.angle + 0.1) % 360;
      }
      if (p.keyIsDown(p.UP_ARROW)) {
        this.ship.vel.x += p.sin(this.ship.angle) * this.ship.speed;
        this.ship.vel.y += -p.cos(this.ship.angle) * this.ship.speed;
      }
      if (touching) {
        const mouse = p.createVector(p.mouseX, p.mouseY);
        const looking = mouse.sub(this.ship.pos);
        this.ship.angle = looking.heading() + p.PI / 2;
        if (Date.now() - touchStart >= 100) {
          this.ship.vel.x += p.sin(this.ship.angle) * this.ship.speed;
          this.ship.vel.y += -p.cos(this.ship.angle) * this.ship.speed;
        }
      }
    }

    updateBullets() {
      this.bullets.reverse().forEach((bullet, i) => {
        bullet.update();
        // Check for bullet hits
        this.asteroids.forEach((asteroid) => {
          if (bullet.hits(asteroid)) {
            bullet.destroyed = true;
            if (asteroid.size > 12) {
              const ast1 = new Asteroid();
              ast1.size = asteroid.size >> 1;
              ast1.pos = asteroid.pos.copy();
              ast1.vel = p5.Vector.random2D();
              ast1.angle = p.random(360);

              const ast2 = new Asteroid();
              ast2.size = asteroid.size >> 1;
              ast2.pos = asteroid.pos.copy();
              ast2.vel = p5.Vector.random2D();
              ast2.angle = p.random(360);

              this.asteroids.push(ast1);
              this.asteroids.push(ast2);
            }
            asteroid.destroyed = true;
            this.score += 100;
          }
        });
        if (bullet.destroyed) {
          this.bullets.splice(i, 1);
        }
      });
    }

    updateAsteroids() {
      // Update asteroids
      this.asteroids.reverse().forEach((asteroid, i) => {
        asteroid.update();
        if (asteroid.destroyed) {
          this.asteroids.splice(i, 1);
        } else if (this.ship.hits(asteroid)) {
          this.shipExploded();
        }
      });
    }

    draw() {
      this.ship.draw();
      this.bullets.forEach((bullet) => bullet.draw());
      this.asteroids.forEach((asteroid) => asteroid.draw());

      // Draw board
      p.textAlign(p.RIGHT);
      p.textSize(40);
      p.text(`${this.score}`, 195, p.textSize());
      for (let i = 0; i < this.lives; ++i) {
        const shape = new Ship();
        shape.pos.y = 60;
        shape.pos.x = 120 + i * 20;
        shape.vel = p.createVector(0, 0);
        shape.angle = 0;
        shape.size = 3;
        shape.draw();
      }
    }

    spawnShip() {
      this.ship.pos = p.createVector(p.width / 2, p.height / 2);
      this.ship.vel = p.createVector(0, 0);
      this.asteroids.forEach((asteroid) => {
        if (this.ship.hits(asteroid)) {
          asteroid.pos.x += asteroid.vel.x * 140;
          asteroid.pos.y += asteroid.vel.y * 140;
        }
      });
    }

    shipExploded() {
      this.lives -= 1;
      this.score -= 500;
      if (this.lives <= 0) {
        this.gameover = true;
      } else {
        this.spawnShip();
      }
    }

    start() {
      this.started = true;
      this.paused = false;
      this.gameover = false;
      this.level = 1;
      this.lives = 4;
      this.score = 0;

      this.spawnShip();

      // Spawn initial asteroids
      const asteroidCount =
        this.asteroids.length > 0
          ? p.min(this.level + 2, this.asteroids.length)
          : this.level + 2;
      this.asteroids.length = 0;
      this.bullets.length = 0;
      for (let i = 0; i < asteroidCount; ++i) {
        this.asteroids.push(new Asteroid(this.ship));
      }

      p.loop();
    }

    stop() {
      this.started = false;
      p.noLoop();
    }

    togglePause() {
      this.paused ? p.loop() : p.noLoop();
      this.paused = !this.paused;
    }

    fire() {
      this.bullets.push(new Bullet(this.ship));
    }

    keyPressed(keyCode: number) {
      switch (keyCode) {
        case p.ENTER: {
          if (!this.started || this.gameover) {
            this.start();
          }
          break;
        }
        case p.ESCAPE:
          this.togglePause();
          break;
        case R:
          this.start();
          break;
      }
    }

    keyReleased(keyCode: number) {
      if (!this.started) return;

      switch (keyCode) {
        case SPACE:
          this.fire();
          break;
      }
    }
  }

  class SpaceObj {
    pos: p5.Vector;
    vel: p5.Vector;
    angle: number;
    size: number;
    speed: number;
    color: string;
    destroyed: boolean;
    wraps: boolean;
    model: number[][];

    constructor() {
      this.pos = p.createVector(p.random(p.width), p.random(p.height));
      this.vel = p.createVector(0, 0);
      this.angle = 0;
      this.size = 1;
      this.speed = 0;
      this.color = "white";
      this.destroyed = false;
      this.wraps = true;
      this.model = [];
    }

    update() {
      this.pos.add(this.vel);
      if (this.wraps) {
        if (this.pos.x < 0) {
          this.pos.x = p.width;
        } else if (this.pos.x > p.width) {
          this.pos.x = 0;
        }
        if (this.pos.y < 0) {
          this.pos.y = p.height;
        } else if (this.pos.y > p.height) {
          this.pos.y = 0;
        }
      } else if (
        this.pos.x < 0 ||
        this.pos.x > p.width ||
        this.pos.y < 0 ||
        this.pos.y > p.height
      ) {
        this.destroyed = true;
      }
    }

    draw() {
      p.push();
      p.translate(this.pos);
      p.rotate(this.angle);
      p.scale(this.size);
      p.strokeWeight(1 / this.size);
      p.stroke(this.color);
      p.fill(0);
      p.beginShape();
      this.model.forEach(([x = 0, y = 0]) => p.vertex(x, y));
      p.endShape(p.CLOSE);
      p.pop();
    }

    hits(obj: SpaceObj) {
      const x = this.pos.x;
      const y = this.pos.y;
      const cx = obj.pos.x;
      const cy = obj.pos.y;
      const r = obj.size;
      return p.pow(x - cx, 2) + p.pow(y - cy, 2) < p.pow(r, 2);
    }
  }

  class Ship extends SpaceObj {
    constructor() {
      super();
      this.size = 5;
      this.speed = 0.15;
      this.model = [
        [0.0, -5.0],
        [-2.5, 2.5],
        [2.5, 2.5],
      ];
      this.pos = p.createVector(p.width / 2, p.height / 2);
    }
  }

  class Bullet extends SpaceObj {
    constructor(ship: Ship) {
      super();
      // TODO - see if we can move this to the tip of the ship
      const shipSin = p.sin(ship.angle);
      const shipCos = -p.cos(ship.angle);
      this.pos = p.createVector(
        ship.pos.x + ship.size * 5 * shipSin,
        ship.pos.y + ship.size * 5 * shipCos,
      );
      this.speed = 6;
      this.vel = p
        .createVector(this.speed * shipSin, this.speed * shipCos)
        .add(ship.vel);
      this.angle = 100;
      this.size = 2;
      this.wraps = false;
      this.model = [
        [0, 0],
        [0, 1],
        [1, 1],
        [1, 0],
      ];
    }
  }

  class Asteroid extends SpaceObj {
    rotation: number;

    constructor(ship?: Ship) {
      super();
      this.size = 48;
      this.pos = p.createVector(p.random(p.width), p.random(p.height));
      this.vel = p5.Vector.random2D();
      this.angle = p.random(360);
      this.color = "yellow";
      this.rotation = p.random(-0.02, 0.02);

      // Create model
      for (let i = 0; i < 20; ++i) {
        const noise = p.random(1) * 0.4 + 0.8;
        const angle = (i / 20.0) * 2.0 * p.PI;
        const x = noise * p.sin(angle);
        const y = noise * p.cos(angle);
        this.model.push([x, y]);
      }

      // Ensure spawn is far enough from ship
      if (ship && this.inSafeZone(ship)) {
        this.pos.x += this.vel.x * 140;
        this.pos.y += this.vel.y * 140;
      }
    }

    update() {
      super.update();
      this.angle += this.rotation;
    }

    inSafeZone(ship: Ship) {
      const x = this.pos.x;
      const y = this.pos.y;
      const cx = ship.pos.x;
      const cy = ship.pos.y;
      const r = ship.size + 20;
      return p.sqrt(p.pow(x - cx, 2) + p.pow(y - cy, 2)) < r;
    }
  }
}
