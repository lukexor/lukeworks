type EducationInfo = {
  start: string;
  end?: string;
  title: string;
  entity: string;
  location: string;
  icon: string;
  bullets: string[];
};

type Props = {
  list: EducationInfo[];
};

const Education = ({ list }: Props) => <>{list}</>;

export default Education;
export type { EducationInfo };
