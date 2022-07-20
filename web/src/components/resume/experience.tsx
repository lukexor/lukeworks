import dayjs from "dayjs";
import customParseFormat from "dayjs/plugin/customParseFormat";
import s from "pages/resume.module.css";
import Ribbon from "./ribbon";

dayjs.extend(customParseFormat);

export type ExperienceInfo = {
  title: string;
  bullets: ExperienceBullet[];
};

export type ExperienceProps = {
  list: ExperienceInfo[];
};

type ExperienceBullet = {
  start?: string;
  end?: string;
  title: string;
  desc: string;
  website?: string;
};

type ExperienceItemProps = {
  experience: ExperienceBullet;
};

const formattedDate = (date?: string) => {
  return date ? dayjs(date).format("MMM YYYY") : "Present";
};

function ExperienceItem({ experience }: ExperienceItemProps) {
  const { title, start, end, desc, website } = experience;

  return (
    <>
      <div className={s.experience}>
        <div>
          <h4>{title}</h4>
          {start && (
            <em>{`${formattedDate(start)} - ${formattedDate(end)}`}</em>
          )}
        </div>
      </div>
      <p className={s.experienceDescription}>
        {desc} {website && <a href={website}>{website}</a>}
      </p>
    </>
  );
}

export default function Experience({ list }: ExperienceProps) {
  const { title, bullets } = list[0] || {}; // Don't have more than just projects for now
  return (
    <section>
      <Ribbon />
      <h3>{title}</h3>
      <section>
        {bullets?.map((experience) => (
          <ExperienceItem key={experience.title} experience={experience} />
        ))}
      </section>
    </section>
  );
}
