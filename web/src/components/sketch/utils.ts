import p5 from "p5";

const defaultMsg = (p: p5) => {
  p.push();
  p.background(0);
  p.fill(255);
  p.noStroke();
  p.textSize(18);
  p.textAlign(p.CENTER);
  p.fill(255);
  p.text("Click or Tap to load", p.width / 2, p.height / 2);
  p.pop();
};

export const awaitClickStart = (p: p5, msg?: () => void, cb?: () => void) => {
  p.cursor(p.HAND);
  p.noLoop();

  msg ? msg() : defaultMsg(p);

  const start = () => {
    p.mousePressed = () => undefined;
    if (!p.isLooping()) {
      p.cursor(p.ARROW);
      cb && cb();
      p.loop();
    }
  };

  p.mousePressed = start;
};
