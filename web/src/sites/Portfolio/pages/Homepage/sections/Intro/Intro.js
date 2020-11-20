import React, { useEffect, useRef, useState } from "react";
import { HashLink } from "react-router-hash-link";
import SitePaths from "sitePaths";
import { copy } from "sites/Portfolio/util/constants";

import {
  Explore,
  ExploreIcon,
  ExploreText,
  Heading,
  Name,
  Splash,
} from "./intro.styles";
import SplashBg from "./SplashBg";
import splashSketch from "./splashSketch";

const Intro = () => {
  const [loadSubtitle, setLoadSubtitle] = useState(0);
  const [sketch, setSketch] = useState();
  const subtitleRef = useRef();

  // Incrementally fades in subtitle elements
  useEffect(() => {
    setTimeout(() => setLoadSubtitle(1), 600);
    setTimeout(() => setLoadSubtitle(2), 1200);
    setTimeout(() => setLoadSubtitle(3), 1800);

    setSketch(new window.p5(splashSketch, "splash"));
    return () => {
      if (sketch) {
        sketch.remove();
      }
    };
  }, []);

  return (
    <>
      <a id="home"></a>
      <SplashBg />
      <Splash id="splash">
        <Heading>
          <h1>
            {copy.Intro.title} <Name>{copy.Contact.firstName}</Name>.
          </h1>
          <h2 ref={subtitleRef}>
            {copy.Intro.subtitle.map((line, i) => (
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
          </h2>
        </Heading>
        <Explore>
          <ExploreText>{copy.Intro.Explore.text}</ExploreText>
          <HashLink smooth to={SitePaths.blog} alt={copy.Intro.Explore.alt}>
            <ExploreIcon icon={copy.Intro.Explore.icon} swapOpacity />
          </HashLink>
        </Explore>
      </Splash>
    </>
  );
};

export default Intro;
