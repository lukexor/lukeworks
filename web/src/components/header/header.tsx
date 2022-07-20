import features from "data/features.json";
import s from "./header.module.css";
import Logo from "./logo";
import Menu from "./menu";
import SearchBox from "./search";

export default function Header() {
  return (
    <div className={s.navbar}>
      <div className={s.navContent}>
        <Logo />
        <div className={s.navItems}>
          {features.search && <SearchBox />}
          <Menu />
        </div>
      </div>
    </div>
  );
}
