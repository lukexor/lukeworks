import "./Intro.css";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import p5 from "p5";
import copy from "portfolio/data/copy.json";
import Icons from "portfolio/Icons";
import { useEffect, useRef, useState } from "react";
import { Link } from "react-router-dom";
import routes from "routes.json";
import SplashBg from "./SplashBg";
import splashSketch from "./splashSketch";

const {
  portfolio: {
    routes: { blog },
  },
} = routes;
const { firstName } = copy.Contact;
const { title, subtitle, explore } = copy.Intro;

const Intro = () => {
  const [loadSubtitle, setLoadSubtitle] = useState(0);
  const [sketch, setSketch] = useState<p5>();
  const subtitleRef = useRef(null);

  // Incrementally fades in subtitle elements
  useEffect(() => {
    setTimeout(() => setLoadSubtitle(1), 600);
    setTimeout(() => setLoadSubtitle(2), 1200);
    setTimeout(() => setLoadSubtitle(3), 1800);

    const splash = document.querySelector("#splash") as HTMLElement;
    splash && setSketch(new window.p5(splashSketch, splash));
    return () => {
      if (sketch) {
        sketch.remove();
      }
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
          <Link to={blog} className="img">
            <FontAwesomeIcon
              className="explore-icon"
              icon={Icons.explore}
              swapOpacity
            />
          </Link>
        </div>
      </section>
    </>
  );
};

export default Intro;
