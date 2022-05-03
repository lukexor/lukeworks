import p5 from "p5";

const pongSketch = (p: p5) => {
  const PADDLE_WIDTH = 15;
  const PADDLE_HEIGHT = 90;
  const PADDLE_PADDING = 30;
  const MAX_BOUNCE_ANGLE = 75;
  const BALL_SPEED = 8;
  const PLAYER_SPEED = 6;
  const CPU_SPEED = 6;

  const LIMIT_TOP = 10 + PADDLE_HEIGHT / 2;
  let LIMIT_BOT: number;

  const BALL_SIZE = 15;
  const TEXT_SIZE = 80;

  type PlayerType = "cpu" | "player1" | "player2";

  const W = 87;
  const R = 82;
  const S = 83;
  const PLAY_CPU = 49;
  const PLAY_PERSON = 50;

  let game: Game;

  p.disableFriendlyErrors = true;

  const intro = () => {
    game = new Game();

    const textX = p.width / 2;
    const textY = 150;
    p.push();
    p.fill(255);
    p.background(0);
    p.textAlign(p.CENTER);
    p.textSize(70);
    p.text("PONG", textX, textY);
    p.textSize(20);
    p.text(
      "CHOOSE OPPONENT\n" + "1: COMPUTER\n" + "2: PERSON\n",
      textX,
      textY + 50
    );
    p.textSize(15);
    p.text(
      "INSTRUCTIONS:\n\n" +
        "UP / DOWN: PLAYER 1 PADDLE\n" +
        "W / S: PLAYER 2 PADDLE\n" +
        "ESCAPE: TOGGLE PAUSE\n" +
        "R: START NEW GAME\n",
      textX,
      textY + 150
    );
    p.pop();
    p.noLoop();
  };

  p.setup = () => {
    p.createCanvas(p.windowWidth, p.windowHeight);

    p.noStroke();
    p.fill(255);
    p.textFont("Courier");
    p.textSize(TEXT_SIZE);
    LIMIT_BOT = p.height - 20 - PADDLE_HEIGHT / 2;

    intro();
  };

  p.draw = () => {
    if (!p.isLooping()) {
      return;
    }
    game.update();
    game.draw();
  };

  p.keyPressed = () => {
    game.keyPressed(p.keyCode);
    return false;
  };

  p.keyReleased = () => {
    game.keyReleased(p.keyCode);
    return false;
  };

  p.touchStarted = () => {
    if (!p.isLooping()) {
      game.playCpu();
    }
    game.touchStarted();
    return false;
  };

  p.touchEnded = () => {
    game.touchEnded();
    return false;
  };

  class Game {
    paused: boolean;
    ball: Ball;
    player1: Player;
    player2: Player;
    touching: boolean;

    constructor() {
      this.paused = false;
      this.ball = new Ball();
      this.player1 = new Player("player1");
      this.player2 = new Player("cpu");
      this.touching = false;
    }

    draw() {
      p.background(51);
      // Draw board
      p.fill(150);
      // Center p.line
      const dotted_size = this.ball.size;
      for (let i = 0; i < p.height; i += 2 * dotted_size) {
        p.square(p.width / 2, i, dotted_size);
      }

      this.ball.draw();
      this.player1.draw();
      this.player2.draw();
    }

    update() {
      this.ball.update(this.player1, this.player2);
      if (this.touching && this.player1.type === "player1") {
        this.player1.pos.y = p.mouseY;
      }
      this.player1.update(this.ball);
      this.player2.update(this.ball);
    }

    keyPressed(keyCode: number) {
      switch (keyCode) {
        case p.ESCAPE:
          this.togglePause();
          return;
        case p.UP_ARROW:
          this.player1.moveUp();
          break;
        case p.DOWN_ARROW:
          this.player1.moveDown();
          break;
        case PLAY_CPU:
          this.playCpu();
          break;
        case PLAY_PERSON:
          this.playPerson();
          break;
        case R:
          intro();
          break;
      }
      if (this.player2.type === "player2") {
        switch (keyCode) {
          case W:
            this.player2.moveUp();
            break;
          case S:
            this.player2.moveDown();
            break;
        }
      }
    }

    keyReleased(keyCode: number) {
      switch (keyCode) {
        case p.UP_ARROW:
        case p.DOWN_ARROW:
          this.player1.stopMovement();
          break;
      }
      if (this.player2.type === "player2") {
        switch (keyCode) {
          case W:
          case S:
            this.player2.stopMovement();
        }
      }
    }

    touchStarted() {
      this.touching = true;
    }
    touchEnded() {
      this.touching = false;
    }

    start() {
      p.loop();
    }
    togglePause() {
      this.paused ? p.loop() : p.noLoop();
      this.paused = !this.paused;
    }

    playCpu() {
      this.player1 = new Player("player1");
      this.player2 = new Player("cpu");
      this.start();
    }
    playPerson() {
      this.player1 = new Player("player1");
      this.player2 = new Player("player2");
      this.start();
    }
  }

  class Player {
    type: PlayerType;
    w: number;
    h: number;
    score: number;
    speed: number;
    pos: p5.Vector;
    vel: p5.Vector;

    constructor(type: PlayerType) {
      this.type = type;
      this.w = PADDLE_WIDTH;
      this.h = PADDLE_HEIGHT;
      this.score = 0;
      this.speed = PLAYER_SPEED;

      let x = PADDLE_PADDING + this.w / 2;
      if (this.type !== "player1") {
        x = p.width - PADDLE_PADDING - this.w / 2;
      }
      if (this.type === "cpu") {
        this.speed = CPU_SPEED;
      }
      const y = p.height / 2 - this.h / 2;
      this.pos = p.createVector(x, y);
      this.vel = p.createVector(0, 0);
    }

    left() {
      return this.pos.x - this.w / 2;
    }

    right() {
      return this.pos.x + this.w / 2;
    }

    top() {
      return this.pos.y - this.h / 2;
    }

    bottom() {
      return this.pos.y + this.h / 2;
    }

    update(ball: Ball) {
      this.pos.add(this.vel);

      if (this.type === "cpu" && ball) {
        if (ball.pos.y > this.pos.y + 1 && ball.vel.y > 0) {
          this.moveDown();
        } else if (ball.pos.y < this.pos.y - 5 && ball.vel.y < 0) {
          this.moveUp();
        } else {
          this.stopMovement();
        }
      }

      if (this.pos.y < LIMIT_TOP) {
        this.pos.y = LIMIT_TOP;
      } else if (this.pos.y > LIMIT_BOT) {
        this.pos.y = LIMIT_BOT;
      }
    }

    draw() {
      p.fill(255);
      p.rect(this.left(), this.top(), this.w, this.h);
      p.fill(150);
      this.drawScore();
    }

    setVelocity(velocity: number) {
      this.vel.y = velocity;
    }

    stopMovement() {
      this.vel.y = 0;
    }

    moveUp() {
      this.vel.y = -this.speed;
    }

    moveDown() {
      this.vel.y = this.speed;
    }

    drawScore() {
      let x = p.width / 2 - TEXT_SIZE + BALL_SIZE / 2;
      p.textAlign(p.RIGHT);
      if (this.type !== "player1") {
        p.textAlign(p.LEFT);
        x = p.width / 2 + BALL_SIZE / 2 + TEXT_SIZE;
      }
      const y = TEXT_SIZE;
      p.text(this.score, x, y);
    }
  }

  class Ball {
    size: number;
    radius: number;
    speed: number;
    pos: p5.Vector;
    vel: p5.Vector;

    constructor() {
      this.size = BALL_SIZE;
      this.radius = this.size / 2;
      this.speed = BALL_SPEED;
      this.pos = p.createVector(0, 0);
      this.vel = p.createVector(0, 0);
      this.resetPos();
    }

    resetPos() {
      this.pos = p.createVector(
        p.width / 2 - this.radius,
        p.height / 2 - this.radius
      );
      this.vel = p5.Vector.fromAngle(p.radians(p.random(-45, 45)));
      if (p.random(1) < 0.5) {
        this.vel.x *= -1;
      }
      this.vel.setMag(this.speed);
    }

    left() {
      return this.pos.x - this.radius;
    }

    right() {
      return this.pos.x + this.radius;
    }

    top() {
      return this.pos.y - this.radius;
    }

    bottom() {
      return this.pos.y + this.radius;
    }

    update(player1: Player, player2: Player) {
      this.pos.add(this.vel);

      // Bounce off paddle
      const reflected1 = this.hits(player1);
      const reflected2 = this.hits(player2);
      if (reflected1 && this.vel.x < 0) {
        this.vel = reflected1;
        this.pos.x = player1.right() + this.radius;
      } else if (reflected2 && this.vel.x > 0) {
        this.vel = reflected2;
        this.pos.x = player2.left() - this.radius;
      } else if (
        (this.top() <= 0 && this.vel.y < 0) ||
        (this.bottom() >= p.height && this.vel.y > 0)
      ) {
        // Bounce off ceiling/floor
        this.vel.y *= -1;
      } else if (this.right() <= 0) {
        // Player2 won
        player2.score += 1;
        this.resetPos();
      } else if (this.left() >= p.width) {
        // Player1 won
        player1.score += 1;
        this.resetPos();
      }
    }

    hits(player: Player) {
      const withinRange =
        this.bottom() >= player.top() && this.top() <= player.bottom();
      let hitPlayer = false;
      const max_angle = MAX_BOUNCE_ANGLE;
      if (player.type === "player1") {
        hitPlayer =
          this.left() < player.right() && this.right() > player.right();
      } else {
        hitPlayer = this.right() > player.left() && this.left() < player.left();
        // max_angle = 180 + max_angle;
      }
      if (withinRange && hitPlayer) {
        const yRelativeIntersect = player.pos.y - this.pos.y;
        const yNormalizedIntersect =
          yRelativeIntersect / ((player.h + this.size) / 2);
        const bounceAngle = -1 * yNormalizedIntersect * max_angle;
        const bounceVel = p5.Vector.fromAngle(p.radians(bounceAngle)).setMag(
          this.speed
        );
        if (player.type !== "player1") {
          bounceVel.x *= -1;
        }
        return bounceVel;
      }
      return null;
    }
    draw() {
      p.fill(255);
      p.rect(this.left(), this.top(), this.size, this.size);
    }
  }
};

export default pongSketch;
