export type HashAnchorProps = {
  id: string;
};

export default function HashAnchor({ id }: HashAnchorProps) {
  return <span id={id.replace("#", "")} className="anchor"></span>;
}
