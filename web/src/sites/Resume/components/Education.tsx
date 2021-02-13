import React from "react";

type EducationItem = {
  start: string;
  end: Maybe<string>;
  title: string;
  entity: string;
  location: string;
  icon: string;
  bullets: string[];
};

type EducationProps = {
  list: EducationItem[];
};

const Education: React.FC<EducationProps> = () => <></>;

export default Education;
