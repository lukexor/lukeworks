import p5 from "p5";
import { awaitClickStart } from "./utils";

export default function lorenzAttractorSketch(p: p5) {
  let x = 0.01;
  let y = 0;
  let z = 0;

  const sig = 10; // a
  const rho = 28; // b
  const beta = 8 / 3.0; // c

  const points: p5.Vector[] = [];
  let font: p5.Font;

  p.disableFriendlyErrors = true;

  p.preload = () => {
    font = p.loadFont("/fonts/noto_sans_regular.ttf");
  };

  p.setup = () => {
    p.createCanvas(p.windowWidth, p.windowHeight, p.WEBGL);
    awaitClickStart(
      p,
      () => {
        p.background(0);
        p.textFont(font);
        p.textSize(18);
        p.textAlign(p.CENTER);
        p.fill(255);
        p.text("Click or Tap to load", 0, 0);
      },
      () => {
        p.colorMode(p.HSB);
        p.noFill();
        p.strokeWeight(2);
      },
    );
  };

  p.draw = () => {
    if (!p.isLooping()) {
      return;
    }

    p.background(0);

    if (points.length >= 4000) {
      points.splice(0);
    }

    const dt = 0.01;
    const dx = sig * (y - x) * dt;
    const dy = (x * (rho - z) - y) * dt;
    const dz = (x * y - beta * z) * dt;
    x += dx;
    y += dy;
    z += dz;

    points.push(p.createVector(x, y, z));

    p.translate(0, 0, -80);
    p.scale(5);
    p.stroke(255);

    p.rotateX(p.millis() / 2000);
    p.rotateY(p.millis() / 4000);

    let hu = 0;
    let increase = true;
    p.beginShape(p.QUAD_STRIP);
    points.forEach((point) => {
      p.stroke(hu, 255, 255);
      p.vertex(point.x, point.y, point.z);
      if (increase) {
        hu += 0.1;
      } else {
        hu -= 0.1;
      }
      if (hu > 255) {
        increase = false;
      } else if (hu < 0) {
        increase = true;
      }
    });
    p.endShape();
  };
}
