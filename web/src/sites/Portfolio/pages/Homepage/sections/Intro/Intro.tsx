import p5 from "p5";
import React, { useEffect, useRef, useState } from "react";
import { HashLink } from "react-router-hash-link";
import copy from "sites/Portfolio/data/copy.json";
import Icons from "sites/Portfolio/Icons";
import routes from "sites/Portfolio/routes";
import {
  Explore,
  ExploreIcon,
  ExploreText,
  Heading,
  Name,
  Splash,
  Subtitle,
} from "./intro.styles";
import SplashBg from "./SplashBg";
import splashSketch from "./splashSketch";

const Intro: React.FC = () => {
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
      <Splash id="splash">
        <Heading>
          <h1>
            {copy.Intro.title} <Name>{copy.Contact.firstName}</Name>.
          </h1>
          <Subtitle ref={subtitleRef}>
            {copy.Intro.subtitle.map((line: string, i: number) => (
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
          </Subtitle>
        </Heading>
        <Explore>
          <ExploreText>{copy.Intro.explore}</ExploreText>
          <HashLink smooth to={routes.blog.path} className="img">
            <ExploreIcon icon={Icons.explore} swapOpacity />
          </HashLink>
        </Explore>
      </Splash>
    </>
  );
};

export default Intro;
