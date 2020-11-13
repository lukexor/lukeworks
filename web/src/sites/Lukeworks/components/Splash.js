import {
  ExploreIcon,
  ExploreText,
  Intro,
  Name,
  StyledExplore,
  StyledSplash,
  SubTitle,
  Title,
} from "./splash.styles";
import React, { useEffect, useRef, useState } from "react";
import { CSSTransition } from "react-transition-group";

const Splash = () => {
  const [loadSubtitle, setLoadSubtitle] = useState(false);
  const nodeRef = useRef(null);

  useEffect(() => {
    setTimeout(() => setLoadSubtitle(true), 1000);
  }, []);

  return (
    <StyledSplash>
      <Intro>
        <Title>
          Hi, I&apos;m <Name>Lucas</Name>.
        </Title>
        <CSSTransition
          nodeRef={nodeRef}
          in={loadSubtitle}
          timeout={1000}
          classNames="fade"
          unmountOnExit
        >
          <SubTitle ref={nodeRef}>
            Software Engineer.
            <br />
            Designer.
            <br />
            Thinker.
          </SubTitle>
        </CSSTransition>
      </Intro>
      <StyledExplore>
        <ExploreText>Please, take a look around!</ExploreText>
        <ExploreIcon className="material-icons md-36" alt="expand more">
          expand_more
        </ExploreIcon>
      </StyledExplore>
    </StyledSplash>
  );
};

export default Splash;
