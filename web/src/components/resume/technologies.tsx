import s from "pages/resume.module.css";

export type TechnologiesProps = {
  list: string[];
};

export default function Technologies({ list }: TechnologiesProps) {
  return (
    <section>
      <h3>Technologies</h3>
      <ul className={s.technology}>
        {list.map((tech) => (
          <li key={tech}>{tech}</li>
        ))}
      </ul>
    </section>
  );
}
