import dayjs from "dayjs";
import customParseFormat from "dayjs/plugin/customParseFormat";

dayjs.extend(customParseFormat);

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

type ExperienceItemProps = {
  experience: ExperienceBullet;
};

type ExperienceProps = {
  list: ExperienceInfo[];
};

const formattedDate = (date?: string) => {
  return date ? dayjs(date).format("MMM YYYY") : "Present";
};

const ExperienceItem = ({ experience }: ExperienceItemProps) => {
  const { title, start, end, desc, website } = experience;

  return (
    <>
      <div className="experience">
        <div>
          <h4>{title}</h4>
          {start && (
            <em>{`${formattedDate(start)} - ${formattedDate(end)}`}</em>
          )}
        </div>
      </div>
      <p className="experience-description">
        {desc} {website && <a href={website}>{website}</a>}
      </p>
    </>
  );
};

const Experience = ({ list }: ExperienceProps) => {
  const { title, bullets } = list[0] || {}; // Don't have more than just projects for now
  return (
    <section className="projects">
      <img className="ribbon" src="/images/resume/ribbon.png" alt="" />
      <h3>{title}</h3>
      <section className="experience-list">
        {bullets?.map((experience) => (
          <ExperienceItem key={experience.title} experience={experience} />
        ))}
      </section>
    </section>
  );
};

export default Experience;
export type { ExperienceInfo };
