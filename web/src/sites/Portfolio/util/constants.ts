import { faGithub, faLinkedin } from "@fortawesome/free-brands-svg-icons";
import {
  faAngleDoubleUp,
  faBars,
  faChevronCircleDown,
  faEnvelope,
  faRssSquare,
  faSearch,
  faTimes,
} from "@fortawesome/free-solid-svg-icons";
import SitePaths from "sitePaths";

const copy = {
  Head: {
    title: "Lucas Petherbridge | Software Engineer",
    description:
      "A blog and project portfolio by Lucas Petherbridge on programming, technology, and video game topics.",
  },
  Logo: {
    text: "LP",
  },
  Search: {
    placeholder: "Search...",
    icon: faSearch,
    clearIcon: faTimes,
  },
  Menu: {
    icon: faBars,
    links: [
      [SitePaths.home, "Home"],
      [SitePaths.blog, "Blog"],
      [SitePaths.projects, "Projects"],
      [SitePaths.about, "About"],
      [SitePaths.contact, "Contact"],
    ],
  },
  Footer: {
    backToTop: {
      icon: faAngleDoubleUp,
    },
    copyright: `© ${new Date().getFullYear()}`,
    rightsReserved: "All Rights Reserved",
    socialIcons: [
      { icon: faGithub, link: "http://github.com/lukexor/", title: "GitHub" },
      {
        icon: faLinkedin,
        link: "https://linkedin.com/in/lucaspetherbridge",
        title: "LinkedIn",
      },
      {
        icon: faRssSquare,
        link: "https://feeds.feedburner.com/LucasPetherbridge",
        title: "RSS Feed",
      },
      { icon: faEnvelope, link: "mailto: me@lukeworks.tech", title: "Email" },
    ],
  },
  Intro: {
    title: "Hi, I'm",
    subtitle: ["Software Engineer.", "Designer.", "Thinker."],
    Explore: {
      text: "Please, take a look around!",
      icon: faChevronCircleDown,
    },
  },
  Contact: {
    firstName: "Lucas",
    lastName: "Petherbridge",
    phone: "5036168265",
    email: "me@lukeworks.tech",
  },
};

export { copy };
