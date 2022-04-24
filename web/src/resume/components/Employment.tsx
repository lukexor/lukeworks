import dayjs from "dayjs";
import customParseFormat from "dayjs/plugin/customParseFormat";
import duration from "dayjs/plugin/duration";

dayjs.extend(duration);
dayjs.extend(customParseFormat);

type PositionInfo = {
  title: string;
  start: string;
  end?: string;
  bullets: string[];
};

type EmploymentInfo = {
  entity: string;
  location: string;
  icon: string;
  positions: PositionInfo[];
};

type PositionProps = {
  position: PositionInfo;
};

type EmploymentItemProps = {
  employment: EmploymentInfo;
};

type EmploymentProps = {
  list: EmploymentInfo[];
};

const calcDuration = (start: string, end?: string) => {
  const startDate = dayjs(start, "YYYY-MM");
  const endDate = end ? dayjs(end, "YYYY-MM") : dayjs();
  return dayjs.duration(endDate.diff(startDate));
};

const formattedDuration = (start: string, end?: string) => {
  const duration = calcDuration(start, end);
  const yrStr = duration.years() == 1 ? "yr" : "yrs";
  const moStr = duration.months() == 1 ? "mo" : "mos";
  return duration.years() > 0 && duration.months() > 0
    ? duration.format(`Y [${yrStr}] M [${moStr}]`)
    : duration.years() > 0
    ? duration.format(`Y [${yrStr}]`)
    : duration.format(`M [${moStr}]`);
};

const formattedDate = (date?: string) => {
  return date ? dayjs(date).format("MMM YYYY") : "Present";
};

const Position = ({ position }: PositionProps) => {
  const { start, end, bullets } = position;
  const duration = formattedDuration(start, end);
  return (
    <li>
      <h5>{position.title}</h5>
      <em>{`${formattedDate(start)} - ${formattedDate(end)} ∙ ${duration}`}</em>
      <ul>
        {bullets.map((bullet) => (
          <li key={bullet}>{bullet}</li>
        ))}
      </ul>
    </li>
  );
};

const EmploymentItem = ({ employment }: EmploymentItemProps) => {
  const { positions, icon, entity, location } = employment;
  const start = positions[positions.length - 1]?.start ?? "";
  const end = positions[0]?.end;
  const duration = formattedDuration(start, end);

  return (
    <>
      <div className="experience">
        <img src={icon} alt={`${employment.entity} Logo`} />
        <div>
          <h4>{entity}</h4>
          <em>
            {duration} ∙ {location}
          </em>
        </div>
      </div>
      <ul className="positions">
        {positions.map((position) => (
          <Position key={position.title} position={position} />
        ))}
      </ul>
    </>
  );
};

const Employment = ({ list }: EmploymentProps) => {
  return (
    <section className="employment">
      <img className="ribbon" src="/images/resume/ribbon.png" alt="" />
      <h3>Work Experience</h3>
      <section className="employment-list">
        {list.map((employment) => (
          <EmploymentItem key={employment.entity} employment={employment} />
        ))}
      </section>
    </section>
  );
};

export default Employment;
export type { EmploymentInfo };
