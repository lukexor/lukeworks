import dayjs from "dayjs";
import customParseFormat from "dayjs/plugin/customParseFormat";
import duration from "dayjs/plugin/duration";
import Image from "next/image";
import s from "pages/styles/resume.module.css";

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
  subentity?: string;
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

const formattedDate = (date?: string) => {
  return date ? dayjs(date).format("MMM YYYY") : "Present";
};

function Position({ position }: PositionProps) {
  const { start, end, bullets } = position;
  return (
    <li>
      <div className={s.experienceEntity}>
        <h5>{position.title}</h5>
        <em>{`${formattedDate(start)} - ${formattedDate(end)}`}</em>
      </div>
      <ul>
        {bullets.map((bullet) => (
          <li key={bullet}>{bullet}</li>
        ))}
      </ul>
    </li>
  );
}

function EmploymentItem({ employment }: EmploymentItemProps) {
  const { positions, icon, entity, subentity, location } = employment;

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
        <div className={s.experienceEntity}>
          <h4>
            {entity}
            {subentity ? <div className={s.subentity}>{subentity}</div> : null}
          </h4>
          <em>{location}</em>
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
      <h3>Work Experience</h3>
      <section className={s.employmentList}>
        {list.map((employment) => (
          <EmploymentItem key={employment.entity} employment={employment} />
        ))}
      </section>
    </section>
  );
}
