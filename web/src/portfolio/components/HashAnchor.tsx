type Props = {
  id: string;
};

const HashAnchor = ({ id }: Props) => (
  <span id={id.replace("#", "")} className="anchor"></span>
);

export default HashAnchor;
