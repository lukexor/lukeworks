type HeaderInfo = string; // TODO

type Props = {
  header: HeaderInfo;
};

const Header = ({ header }: Props) => <>{header}</>;

export default Header;
export type { HeaderInfo };
