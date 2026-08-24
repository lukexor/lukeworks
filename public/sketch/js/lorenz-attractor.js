import { awaitClickStart } from "./utils.js";

export default function lorenzAttractorSketch(p) {
  let x = 0.01;
  let y = 0;
  let z = 0;

  const sig = 10; // a
  const rho = 28; // b
  const beta = 8 / 3.0; // c

  const points = [];

  p.disableFriendlyErrors = true;

  p.setup = () => {
    p.createCanvas(p.windowWidth, p.windowHeight, p.WEBGL);
    awaitClickStart(
      p,
      () => {
        // A WEBGL canvas can only draw text from a font file. The prompt is
        // drawn into a 2D buffer, which has the default font, and blitted on.
        // WEBGL puts the origin at the centre, hence the negative corner.
        const prompt = p.createGraphics(p.width, p.height);
        prompt.textSize(18);
        prompt.textAlign(p.CENTER, p.CENTER);
        prompt.fill(255);
        prompt.text("Click or Tap to load", p.width / 2, p.height / 2);

        p.background(0);
        p.image(prompt, -p.width / 2, -p.height / 2);
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
    // No shape mode: the attractor is one open polyline. QUAD_STRIP pairs the
    // vertices up and reads past the end of an odd-length list, which throws
    // inside p5 on the first frame, when there is a single point.
    p.beginShape();
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
