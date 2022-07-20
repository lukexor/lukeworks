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
    <header className="header" role="banner">
      <section className="title">
        <h1 className="name">{header.name}</h1>
        <h2 className="position">{header.position}</h2>
      </section>
      <p className="summary">{header.summary}</p>
    </header>
  );
}
