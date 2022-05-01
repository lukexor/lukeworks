import "./Logo.css";
import { Link, useLocation } from "react-router-dom";
import routes from "routes.json";
import copy from "../../data/copy.json";

const { home } = routes;
const { text } = copy.Logo;

const Logo = () => {
  const location = useLocation();
  if (location.pathname === home.path) {
    return (
      <a href={`${home.path}#`} className="logo">
        {text}
      </a>
    );
  } else {
    return (
      <Link to={home.path} className="logo">
        {text}
      </Link>
    );
  }
};

export default Logo;
