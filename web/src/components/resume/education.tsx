import Image from "next/image";
import s from "pages/styles/resume.module.css";
import Ribbon from "./ribbon";

export type EducationInfo = {
  start: string;
  end?: string;
  title: string;
  entity: string;
  location: string;
  icon: string;
  bullets: string[];
};

export type EducationProps = {
  list: EducationInfo[];
};

type EducationItemProps = {
  education: EducationInfo;
};

function EducationItem({ education }: EducationItemProps) {
  const { start, end, title, entity, location, icon, bullets } = education;

  return (
    <>
      <div className={s.experience}>
        <div className={s.experienceImg}>
          <Image src={icon} width={35} height={35} alt={`${entity} Logo`} />
        </div>
        <div>
          <h4>{entity}</h4>
          <span className={s.educationTitle}>{title}</span>
          <br />
          <em>
            {start} - {end} ∙ {location}
          </em>
        </div>
      </div>
      <ul className={s.bullets}>
        {bullets.map((bullet) => (
          <li key={bullet}>{bullet}</li>
        ))}
      </ul>
    </>
  );
}

export default function Education({ list }: EducationProps) {
  return (
    <section>
      <Ribbon />
      <h3>Education</h3>
      <section>
        {list.map((education) => (
          <EducationItem key={education.entity} education={education} />
        ))}
      </section>
    </section>
  );
}
