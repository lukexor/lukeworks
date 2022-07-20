import clsx from "clsx";
import { ExploreIcon } from "components/icons";
import ls from "components/layout/layout.module.css";
import LinkIcon from "components/linkIcon";
import contactCopy from "data/contact.json";
import introCopy from "data/intro.json";
import routes from "data/routes.json";
import { useEffect, useRef, useState } from "react";
import s from "./intro.module.css";
import SplashBg from "./splashBg";

export default function Intro() {
  const [loadSubtitle, setLoadSubtitle] = useState(0);
  const containerRef = useRef(null);
  // const sketchRef = useRef<null | p5>(null);
  const subtitleRef = useRef(null);

  useEffect(() => {
    // Incrementally fades in subtitle elements
    setTimeout(() => setLoadSubtitle(1), 300);
    setTimeout(() => setLoadSubtitle(2), 600);
    setTimeout(() => setLoadSubtitle(3), 900);
  }, []);

  // useEffect(() => {
  //   let mounted = true;
  //   if (mounted && containerRef.current) {
  //     sketchRef.current = new p5(splashSketch, containerRef.current);
  //   }
  //   return () => {
  //     mounted = false;
  //     sketchRef.current?.remove();
  //   };
  // }, []);

  return (
    <>
      <SplashBg />
      <section ref={containerRef} id="splash" className={s.splash}>
        <header className={s.header}>
          <h1>
            {introCopy.title}{" "}
            <span className={s.name}>{contactCopy.firstName}</span>.
          </h1>
          <p className={s.subtitle} ref={subtitleRef}>
            {introCopy.subtitle.map((line: string, i: number) => (
              <span
                key={line}
                className={clsx(
                  ls.fadeEnter,
                  loadSubtitle > i && ls.fadeEnterActive,
                )}
              >
                {line}
                <br />
              </span>
            ))}
          </p>
        </header>
        <div className={s.explore}>
          <p>{introCopy.explore.text}</p>
          <LinkIcon
            href={routes.menu.blog.hash}
            title={introCopy.explore.title}
            icon={ExploreIcon}
            swapOpacity
          />
        </div>
      </section>
    </>
  );
}
