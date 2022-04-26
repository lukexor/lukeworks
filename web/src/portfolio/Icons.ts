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

const Icons = {
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

type IconList = typeof Icons;

const isIcon = (key: string): key is keyof IconList => {
  return key in Icons;
};

export default Icons;
export { isIcon };
export type { IconList };
