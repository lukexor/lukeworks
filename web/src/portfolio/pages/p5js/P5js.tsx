import "./P5js.css";
import useEventListener from "hooks/useEventListener";
import p5 from "p5";
import { useEffect, useRef } from "react";
import { useParams } from "react-router-dom";

// Ignore scrolling keys.
const keysToIgnore = [
  "Space",
  "ArrowUp",
  "ArrowDown",
  "ArrowLeft",
  "ArrowRight",
];

type Params = {
  title: string;
};

const P5js = () => {
  const { title } = useParams<Params>();
  const containerRef = useRef(null);
  const sketchRef = useRef<null | p5>(null);

  useEventListener("keydown", (evt: KeyboardEvent) => {
    if (keysToIgnore.indexOf(evt.code) > -1) {
      evt.preventDefault();
    }
  });

  useEffect(() => {
    let mounted = true;
    const loadSketch = async () => {
      try {
        const sketch = await import(`./sketches/${title}`);
        if (mounted && containerRef.current) {
          sketchRef.current = new window.p5(
            sketch.default,
            containerRef.current
          );
        }
      } catch (err) {
        console.error(err);
      }
    };
    loadSketch();
    return () => {
      mounted = false;
      sketchRef.current?.remove();
    };
  }, []);

  return <section ref={containerRef}></section>;
};

export default P5js;
