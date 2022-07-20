import dayjs from "dayjs";
import customParseFormat from "dayjs/plugin/customParseFormat";
import duration from "dayjs/plugin/duration";
import Image from "next/image";
import s from "pages/resume.module.css";
import Ribbon from "./ribbon";

dayjs.extend(duration);
dayjs.extend(customParseFormat);

export type PositionInfo = {
  title: string;
  start: string;
  end?: string;
  bullets: string[];
};

export type EmploymentInfo = {
  entity: string;
  location: string;
  icon: string;
  positions: PositionInfo[];
};

export type EmploymentProps = {
  list: EmploymentInfo[];
};

type PositionProps = {
  position: PositionInfo;
};

type EmploymentItemProps = {
  employment: EmploymentInfo;
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

function Position({ position }: PositionProps) {
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
}

function EmploymentItem({ employment }: EmploymentItemProps) {
  const { positions, icon, entity, location } = employment;
  const start = positions[positions.length - 1]?.start ?? "";
  const end = positions[0]?.end;
  const duration = formattedDuration(start, end);

  return (
    <>
      <div className={s.experience}>
        <div className={s.experienceImg}>
          <Image
            src={icon}
            width={35}
            height={35}
            alt={`${employment.entity} Logo`}
          />
        </div>
        <div>
          <h4>{entity}</h4>
          <em>
            {duration} ∙ {location}
          </em>
        </div>
      </div>
      <ul className={s.positions}>
        {positions.map((position) => (
          <Position key={position.title} position={position} />
        ))}
      </ul>
    </>
  );
}

export default function Employment({ list }: EmploymentProps) {
  return (
    <section>
      <Ribbon />
      <h3>Work Experience</h3>
      <section className={s.employmentList}>
        {list.map((employment) => (
          <EmploymentItem key={employment.entity} employment={employment} />
        ))}
      </section>
    </section>
  );
}
