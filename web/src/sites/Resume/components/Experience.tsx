import React from "react";

type ExperienceBullet = {
  start: string;
  end: Maybe<string>;
  title: string;
  desc: string;
  website: Maybe<string>;
};

type ExperienceItem = {
  title: string;
  bullets: ExperienceBullet[];
};

type ExperienceProps = {
  list: ExperienceItem[];
};

const Experience: React.FC<ExperienceProps> = () => <></>;

export default Experience;
