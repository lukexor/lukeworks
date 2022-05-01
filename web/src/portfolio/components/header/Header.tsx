import "./Header.css";
import features from "../../data/features.json";
import Logo from "./Logo";
import Menu from "./Menu";
import SearchBox from "./Search";

const Header = () => {
  return (
    <div className="heading">
      <div className="header-bar">
        <Logo />
        <div className="header-icons">
          {features.search && <SearchBox />}
          <Menu />
        </div>
      </div>
    </div>
  );
};

export default Header;
