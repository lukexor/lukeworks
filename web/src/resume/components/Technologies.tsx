type Props = {
  list: string[];
};

const Technologies = ({ list }: Props) => {
  return (
    <section>
      <h3>Technologies</h3>
      <ul className="technology">
        {list.map((tech) => (
          <li key={tech}>{tech}</li>
        ))}
      </ul>
    </section>
  );
};

export default Technologies;
