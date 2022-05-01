import "./Footer.css";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import copy from "portfolio/data/copy.json";
import Icons, { isIcon } from "portfolio/Icons";

const { copyright, socialIcons, rightsReserved, backToTop } = copy.Footer;
const { firstName, lastName } = copy.Contact;

const Footer = () => {
  return (
    <footer>
      <div className="footer-bar">
        <a href="#top" title={backToTop} className="to-top-link">
          <FontAwesomeIcon
            className="link-icon"
            title={backToTop}
            icon={Icons.backToTop}
          />
        </a>
        <div className="social-icons">
          {socialIcons.map(({ icon, link, title }) => {
            return isIcon(icon) ? (
              <a key={link} href={link} title={title}>
                <FontAwesomeIcon
                  title={title}
                  className="link-icon"
                  icon={Icons[icon]}
                />
              </a>
            ) : null;
          })}
        </div>
        <p className="copyright">
          {copyright} {new Date().getFullYear()}{" "}
          <span className="bold">
            {firstName} {lastName}
          </span>
          . {rightsReserved}.
        </p>
      </div>
    </footer>
  );
};

export default Footer;
