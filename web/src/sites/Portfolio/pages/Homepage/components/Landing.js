import React, { useEffect, useRef, useState } from "react";
import SitePaths from "sitePaths";
import { copy } from "util/constants";
import { scrollWithHeaderOffset } from "util/scroll";

import splashSketch from "../lib/splashSketch";
import {
  Explore,
  ExploreLink,
  ExploreText,
  Intro,
  Name,
  Splash,
  SubTitle,
  Title,
} from "./landing.styles";

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
    <Splash id="splash">
      <Intro>
        <Title>
          {copy.Intro.title} <Name>{copy.Contact.firstName}</Name>.
        </Title>
        <SubTitle ref={subtitleRef}>
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
        </SubTitle>
      </Intro>
      <Explore>
        <ExploreText>{copy.Explore.text}</ExploreText>
        <ExploreLink
          smooth
          to={SitePaths.blog}
          className="material-icons md-36"
          alt={copy.Explore.logo.alt}
          scroll={scrollWithHeaderOffset}
        >
          {copy.Explore.logo.icon}
        </ExploreLink>
      </Explore>
    </Splash>
  );
};

export default Landing;
