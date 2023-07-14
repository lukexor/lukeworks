import s from "pages/styles/resume.module.css";

export type TechnologiesProps = {
  list: string[];
};

export default function Technologies({ list }: TechnologiesProps) {
  return (
    <section className={s.technologies}>
      <h3>Technologies</h3>
      {list.map((tech) => (
        <div key={tech}>
          {tech}
          <br />
        </div>
      ))}
    </section>
  );
}
