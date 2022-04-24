type ExperienceBullet = {
  start?: string;
  end?: string;
  title: string;
  desc: string;
  website?: string;
};

type ExperienceInfo = {
  title: string;
  bullets: ExperienceBullet[];
};

type Props = {
  list: ExperienceInfo[];
};

const Experience = ({ list }: Props) => <>{list}</>;

export default Experience;
export type { ExperienceInfo };
