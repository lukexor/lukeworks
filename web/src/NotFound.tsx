import "./NotFound.css";
import copy from "portfolio/data/copy.json";
import { Link } from "react-router-dom";
import routes from "routes.json";

const { home } = routes;
const { title, message, action1, action2, linkText1, linkText2 } =
  copy.NotFound;
const { email } = copy.Contact;

const NotFound = () => {
  return (
    <section className="not-found">
      <h1>{title}</h1>
      <p>
        {message}
        <br />
        {action1}
        <a href={`mailto:${email}`} title={email}>
          {linkText1}
        </a>
        {action2}
        <Link to={home.path} title={home.title}>
          {linkText2}
        </Link>
        .
      </p>
    </section>
  );
};

export default NotFound;
