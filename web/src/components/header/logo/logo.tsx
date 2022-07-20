import copy from "data/layout.json";
import routes from "data/routes.json";
import Link from "next/link";
import s from "./logo.module.css";

export default function Logo() {
  return (
    <Link href={routes.home.path}>
      <a className={s.logo}>{copy.logo}</a>
    </Link>
  );
}
