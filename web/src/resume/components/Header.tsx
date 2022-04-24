import "./Header.css";

type HeaderInfo = {
  name: string;
  position: string;
  summary: string;
};

type Props = {
  header: HeaderInfo;
};

const Header = ({ header }: Props) => {
  return (
    <header className="header" role="banner">
      <section className="title">
        <h1 className="name">{header.name}</h1>
        <h2 className="position">{header.position}</h2>
      </section>
      <p className="summary">{header.summary}</p>
    </header>
  );
};

export default Header;
export type { HeaderInfo };
