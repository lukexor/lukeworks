import p5 from "p5";
import { awaitClickStart } from "./utils";

export default function matrixSketch(p: p5) {
  // Global Constants
  const BG_COLOR = [0, 0, 0, 150];

  // Glyph Constants
  const GLYPH_FONT = "Courier New";
  const GLYPH_SIZE = 16;
  const GLYPH_COLOR = [0, 255, 70];
  const GLYPH_COLOR_DARK = [0, 155, 0];
  const GLYPH_HIGHLIGHT = [200, 255, 200];
  const GLYPH_HIGHLIGHT_PROBABILITY = 25;
  const MORPH_PROBABILITY = 10;
  const MORPH_INTERVAL_MIN = 2;
  const MORPH_INTERVAL_MAX = 20;

  // Stream Constants
  const START_Y_MIN = -2000;
  const START_Y_MAX = -200;
  const SPAWN_Y_MIN = -500;
  const SPAWN_Y_MAX = -100;
  const SPEED_MIN = 0.1;
  const SPEED_MAX = 0.7;
  const STREAM_EMPTY_PROBABILITY = 5;
  const STREAM_MIN = 3;
  const STREAM_MAX = 30;

  // Global variables
  const streams: Stream[] = [];
  const chars = [" ", "$", "@", ":", "-", "+", "*", ";", ".", "<", ">"];

  p.disableFriendlyErrors = true;

  p.preload = () => {};
  p.setup = () => {
    p.createCanvas(p.windowWidth, p.windowHeight);
    p.fill(255);

    awaitClickStart(p, undefined, () => {
      p.background(BG_COLOR);
      p.textStyle(p.BOLD);
      p.textFont(GLYPH_FONT);
      p.frameRate(30);

      for (let i = 0; i < 10; ++i) {
        chars.push(i.toString());
      }
      for (let i = 0; i < 96; ++i) {
        chars.push(String.fromCharCode(0x30a0 + i));
      }

      let x = 0;
      for (let i = 0; i < p.floor(p.width / GLYPH_SIZE) - 1; ++i) {
        const y = p.random(START_Y_MIN, START_Y_MAX);
        const stream = new Stream(x, y);
        streams.push(stream);
        x += GLYPH_SIZE;
      }
    });
  };

  p.draw = () => {
    if (!p.isLooping()) {
      return;
    }
    p.background(BG_COLOR);
    streams.reverse().forEach((stream, i) => {
      if (stream.y >= p.height + stream.height) {
        streams.splice(i, 1);
      } else {
        stream.draw();
      }
    });
  };

  class Glyph {
    x: number;
    y: number;
    value: string;
    size: number;
    color: number[];
    morphInterval: number;

    constructor(x: number, y: number) {
      this.x = x;
      this.y = y;
      this.value = "";
      this.size = GLYPH_SIZE;
      this.color = GLYPH_COLOR;
      this.morphInterval = p.round(
        p.random(MORPH_INTERVAL_MIN, MORPH_INTERVAL_MAX),
      );
      this.randomizeGlyph();
    }

    randomizeGlyph() {
      this.value = chars[p.round(p.random(0, chars.length))] || "";
    }

    draw() {
      const morphRoll = p.random(0, 100);
      if (
        p.frameCount % this.morphInterval === 0 &&
        morphRoll <= MORPH_PROBABILITY
      ) {
        this.randomizeGlyph();
      }
      p.fill(this.color);
      p.textSize(this.size);
      p.text(this.value, this.x, this.y);
    }
  }

  class Stream {
    x: number;
    y: number;
    height: number;
    highlight: boolean;
    glyphs: Glyph[];
    glyphSize: number;
    glyphColor: number[];
    speed: number;
    spawned: boolean;

    constructor(x: number, y: number) {
      this.x = x;
      this.y = y;
      this.height = 0;
      this.highlight = false;
      this.glyphs = [];
      this.glyphSize = GLYPH_SIZE;
      this.glyphColor = GLYPH_COLOR;
      this.speed = 0;
      this.spawned = false;
      this.randomizeStream();
    }

    randomizeStream() {
      this.glyphs = [];
      this.speed = p.random(SPEED_MIN, SPEED_MAX);
      if (this.speed === SPEED_MIN) {
        this.glyphColor = GLYPH_COLOR_DARK;
      }

      if (p.round(p.random(0, 100)) <= GLYPH_HIGHLIGHT_PROBABILITY) {
        this.highlight = true;
      }

      const emptyProb = p.round(p.random(0, 100)) <= STREAM_EMPTY_PROBABILITY;
      if (!emptyProb) {
        const numGlyphs = p.round(p.random(STREAM_MIN, STREAM_MAX));
        let y = this.y;
        for (let i = 0; i < numGlyphs; ++i) {
          const glyph = new Glyph(this.x, y);
          glyph.size = this.glyphSize;
          glyph.color = this.glyphColor;
          this.glyphs.push(glyph);
          y -= glyph.size;
          this.height += glyph.size;
        }
      }
    }
    draw() {
      this.y += this.speed * this.glyphSize;
      if (this.y < 0 || this.y >= p.height + this.height) {
        return;
      }
      for (let i = 0; i < this.glyphs.length; ++i) {
        const glyph = this.glyphs[i];
        if (glyph) {
          if (i === 0 && this.highlight) {
            glyph.color = GLYPH_HIGHLIGHT;
          } else {
            glyph.color = this.glyphColor;
          }
          glyph.y = p.floor(this.y) - i * this.glyphSize;
          glyph.draw();
        }
      }
      if (!this.spawned && this.y >= (3 * p.height) / 4) {
        const y = p.random(SPAWN_Y_MIN, SPAWN_Y_MAX);
        const stream = new Stream(this.x, y);
        streams.push(stream);
        this.spawned = true;
      }
    }
  }
}
