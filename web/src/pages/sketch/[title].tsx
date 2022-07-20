import Loading from "components/loading";
import fs from "fs/promises";
import useEventListener from "hooks/useEventListener";
import { GetStaticPropsContext, GetStaticPropsResult } from "next";
import type p5 from "p5";
import path from "path";
import { ParsedUrlQuery } from "querystring";
import { useEffect, useRef } from "react";

type SketchProps = {
  title: string;
};

// Get list of sketches
const getSketches = async () => {
  const filePath = path.join(process.cwd(), "src/data/sketches.json");
  try {
    const jsonData = await fs.readFile(filePath);
    const sketches: string[] = JSON.parse(jsonData.toString());
    return sketches;
  } catch (err: unknown) {
    console.error(err);
    return [];
  }
};

interface GetStaticPropsParams extends ParsedUrlQuery {
  title: string;
}
type GetStaticPathsParams = { params: GetStaticPropsParams };

// Ignore scrolling keys.
const keysToIgnore = [
  "Space",
  "ArrowUp",
  "ArrowDown",
  "ArrowLeft",
  "ArrowRight",
];

export async function getStaticPaths() {
  const sketch = await getSketches();
  const paths = sketch.reduce<GetStaticPathsParams[]>((paths, title) => {
    paths.push({ params: { title } });
    return paths;
  }, []);
  return {
    paths,
    fallback: false,
  };
}

export async function getStaticProps(
  ctx: GetStaticPropsContext<GetStaticPropsParams>,
): Promise<GetStaticPropsResult<SketchProps>> {
  if (!ctx.params) {
    return { notFound: true };
  }

  const sketches = await getSketches();
  const { title } = ctx.params;
  const matchTitle = sketches.find((sketchTitle) => sketchTitle === title);

  if (matchTitle) {
    return { props: { title: matchTitle } };
  } else {
    return {
      notFound: true,
    };
  }
}

export default function Sketch({ title }: SketchProps) {
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
        if (mounted && containerRef.current) {
          const p5 = await import("p5");
          const sketch = await import(`../../components/sketch/${title}`);
          sketchRef.current = new p5.default(
            sketch.default,
            containerRef.current,
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
  }, [title]);

  return (
    <section ref={containerRef}>
      <div id="p5_loading">
        <Loading />
      </div>
    </section>
  );
}
