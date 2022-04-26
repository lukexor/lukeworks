import "./Logo.css";
import copy from "../../data/copy.json";

const Logo = () => (
  <a href="/" className="img-link logo">
    {copy.Logo.text}
  </a>
);

export default Logo;
