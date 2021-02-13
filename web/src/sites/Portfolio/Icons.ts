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

type IconMap = {
  [key: string]: typeof faGithub;
}

const Icons: IconMap = {
  search: faSearch,
  clearField: faTimes,
  menu: faBars,
  backToTop: faAngleDoubleUp,
  github: faGithub,
  linkedIn: faLinkedin,
  rss: faRssSquare,
  email: faEnvelope,
  explore: faChevronCircleDown,
};

export default Icons;
