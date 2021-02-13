import React from "react";

type Position = {
  start: string;
  end?: string;
  title: string;
  entity: string;
  location: string;
  icon: string;
  bullets: string[];
};

type EmploymentProps = {
  list: Position[];
};

const Employment: React.FC<EmploymentProps> = () => <></>;

export default Employment;
