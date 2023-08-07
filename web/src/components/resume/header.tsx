import s from "pages/styles/resume.module.css";

export type HeaderInfo = {
  name: string;
  position: string;
  summary: string;
};

export type HeaderProps = {
  header: HeaderInfo;
};

export default function Header({ header }: HeaderProps) {
  return (
    <header className={s.header} role="banner">
      <section className={s.title}>
        <h1 className={s.name}>{header.name}</h1>
        <h2 className={s.position}>{header.position}</h2>
      </section>
    </header>
  );
}
