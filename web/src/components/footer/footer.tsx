import { BackToTopIcon, isSocialIcon, SocialIcons } from "components/icons";
import LinkIcon from "components/linkIcon";
import contactCopy from "data/contact.json";
import footerCopy from "data/footer.json";
import useIsMounted from "hooks/useIsMounted";
import s from "./footer.module.css";

const { backToTop, copyright, rightsReserved, socialIcons } = footerCopy;
const { firstName, lastName } = contactCopy;

const Footer = () => {
  const isMounted = useIsMounted();

  return (
    <footer className={s.footer}>
      <div className={s.footerContent}>
        {isMounted && (
          <LinkIcon
            href="#"
            className={s.backToTop}
            title={backToTop}
            icon={BackToTopIcon}
          />
        )}
        <div className={s.socialIcons}>
          {socialIcons.map(
            ({ icon, link, title }) =>
              isSocialIcon(icon) && (
                <LinkIcon
                  key={title}
                  href={link}
                  className="link-icon"
                  title={title}
                  icon={SocialIcons[icon]}
                />
              ),
          )}
        </div>
        <p className={s.copyright}>
          {copyright} {new Date().getFullYear()}{" "}
          <span className={s.name}>
            {firstName} {lastName}
          </span>
          . {rightsReserved}.
        </p>
      </div>
    </footer>
  );
};

export default Footer;
