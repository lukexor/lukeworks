type EducationInfo = {
  start: string;
  end?: string;
  title: string;
  entity: string;
  location: string;
  icon: string;
  bullets: string[];
};

type EducationItemProps = {
  education: EducationInfo;
};

type EducationProps = {
  list: EducationInfo[];
};

const EducationItem = ({ education }: EducationItemProps) => {
  const { start, end, title, entity, location, icon, bullets } = education;

  return (
    <>
      <div className="experience">
        <img src={icon} alt={`${entity} Logo`} />
        <div>
          <h4>{entity}</h4>
          <span className="education-title">{title}</span>
          <br />
          <em>
            {start} - {end} ∙ {location}
          </em>
        </div>
      </div>
      <ul className="bullets">
        {bullets.map((bullet) => (
          <li key={bullet}>{bullet}</li>
        ))}
      </ul>
    </>
  );
};

const Education = ({ list }: EducationProps) => {
  return (
    <section className="education">
      <img className="ribbon" src="/images/resume/ribbon.png" alt="" />
      <h3>Education</h3>
      <section className="education-list">
        {list.map((education) => (
          <EducationItem key={education.entity} education={education} />
        ))}
      </section>
    </section>
  );
};

export default Education;
export type { EducationInfo };
