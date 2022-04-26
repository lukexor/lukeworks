import "./Footer.css";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import copy from "portfolio/data/copy.json";
import Icons, { isIcon } from "portfolio/Icons";
import routes from "routes.json";

const {
  portfolio: {
    sections: { home },
  },
} = routes;

const Footer = () => {
  return (
    <footer>
      <div className="footer-bar">
        <a href={home.path} className="to-top-link img-link">
          <FontAwesomeIcon className="to-top-icon" icon={Icons.backToTop} />
        </a>
        <div className="social-icons">
          {copy.Footer.socialIcons.map(({ icon, link, title }) => {
            return isIcon(icon) ? (
              <a key={link} href={link} title={title} className="img-link">
                <FontAwesomeIcon className="social-icon" icon={Icons[icon]} />
              </a>
            ) : null;
          })}
        </div>
        <p className="copyright">
          {copy.Footer.copyright} {new Date().getFullYear()}{" "}
          <span className="bold">
            {copy.Contact.firstName} {copy.Contact.lastName}
          </span>
          . {copy.Footer.rightsReserved}.
        </p>
      </div>
    </footer>
  );
};

export default Footer;
