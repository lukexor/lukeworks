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
    alt: "Homepage",
    text: "LP",
  },
  Search: {
    alt: "Search...",
    icon: faSearch,
    clearAlt: "Clear Search",
    clearIcon: faTimes,
  },
  Menu: {
    alt: "Navigation Menu",
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
      alt: "Back to Top",
      icon: faAngleDoubleUp,
    },
    copyright: `© ${new Date().getFullYear()}`,
    rightsReserved: "All Rights Reserved",
    socialIcons: [
      [faGithub, "http://github.com/lukexor/", "GitHub"],
      [faLinkedin, "https://linkedin.com/in/lucaspetherbridge", "LinkedIn"],
      [
        faRssSquare,
        "https://feeds.feedburner.com/LucasPetherbridge",
        "RSS Feed",
      ],
      [faEnvelope, "mailto: me@lukeworks.tech", "Email"],
    ],
  },
  Intro: {
    title: "Hi, I'm",
    subtitle: ["Software Engineer.", "Designer.", "Thinker."],
    Explore: {
      text: "Please, take a look around!",
      alt: "expand more",
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
