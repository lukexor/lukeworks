type EmploymentInfo = {
  start: string;
  end?: string;
  title: string;
  entity: string;
  location: string;
  icon: string;
  bullets: string[];
};

type Props = {
  list: EmploymentInfo[];
};

const Employment = ({ list }: Props) => <>{list}</>;

export default Employment;
export type { EmploymentInfo };
