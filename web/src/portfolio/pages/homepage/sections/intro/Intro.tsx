import "./Intro.css";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import p5 from "p5";
import copy from "portfolio/data/copy.json";
import Icons from "portfolio/Icons";
import { useEffect, useRef, useState } from "react";
import routes from "routes.json";
import SplashBg from "./SplashBg";
import splashSketch from "./splashSketch";

const {
  portfolio: {
    sections: { blog },
  },
} = routes;
const { firstName } = copy.Contact;
const { title, subtitle, explore } = copy.Intro;

const Intro = () => {
  const [loadSubtitle, setLoadSubtitle] = useState(0);
  const sketchRef = useRef<null | p5>(null);
  const subtitleRef = useRef(null);

  // Incrementally fades in subtitle elements
  useEffect(() => {
    setTimeout(() => setLoadSubtitle(1), 600);
    setTimeout(() => setLoadSubtitle(2), 1200);
    setTimeout(() => setLoadSubtitle(3), 1800);

    const splash: null | HTMLElement = document.querySelector("#splash");
    if (splash) {
      sketchRef.current = new window.p5(splashSketch, splash);
    }
    return () => {
      sketchRef.current?.remove();
    };
  }, []);

  return (
    <>
      <SplashBg />
      <section id="splash" className="splash">
        <header className="header">
          <h1>
            {title} <span className="name">{firstName}</span>.
          </h1>
          <p className="subtitle" ref={subtitleRef}>
            {subtitle.map((line: string, i: number) => (
              <span
                key={`subtitle-${i}`}
                className={`fade-enter ${
                  loadSubtitle > i && "fade-enter-active"
                }`}
              >
                {line}
                <br />
              </span>
            ))}
          </p>
        </header>
        <div className="explore">
          <p>{explore}</p>
          <a href={`${blog.path}${blog.hash}`} className="img-link">
            <FontAwesomeIcon
              className="explore-icon ripple"
              icon={Icons.explore}
              swapOpacity
            />
          </a>
        </div>
      </section>
    </>
  );
};

export default Intro;
