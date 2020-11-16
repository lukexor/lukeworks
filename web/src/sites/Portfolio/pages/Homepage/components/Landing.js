import React, { useEffect, useRef, useState } from "react";
import { HashLink } from "react-router-hash-link";
import SitePaths from "sitePaths";
import { copy } from "sites/Portfolio/util/constants";

import splashSketch from "../lib/splashSketch";
import {
  Explore,
  ExploreIcon,
  ExploreText,
  Intro,
  Name,
  Splash,
} from "./landing.styles";
import SplashBg from "./SplashBg";

const Landing = () => {
  const [loadSubtitle, setLoadSubtitle] = useState(0);
  const subtitleRef = useRef(null);

  // Incrementally fades in subtitle elements
  useEffect(() => {
    setTimeout(() => setLoadSubtitle(1), 600);
    setTimeout(() => setLoadSubtitle(2), 1200);
    setTimeout(() => setLoadSubtitle(3), 1800);

    new window.p5(splashSketch, "splash");
  }, []);

  return (
    <>
      <a id="home"></a>
      <SplashBg />
      <Splash id="splash">
        <Intro>
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
        </Intro>
        <Explore>
          <ExploreText>{copy.Explore.text}</ExploreText>
          <HashLink smooth to={SitePaths.blog} alt={copy.Explore.alt}>
            <ExploreIcon icon={copy.Explore.icon} swapOpacity />
          </HashLink>
        </Explore>
      </Splash>
    </>
  );
};

export default Landing;
