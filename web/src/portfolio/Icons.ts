import { faGithub, faLinkedin } from "@fortawesome/free-brands-svg-icons";
import {
  faAngleDoubleUp,
  faAngleDown,
  faBars,
  faCircleChevronDown,
  faCircleChevronLeft,
  faCircleChevronRight,
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
  explore: faCircleChevronDown,
  previous: faCircleChevronLeft,
  next: faCircleChevronRight,
  more: faAngleDown,
};

type IconList = typeof Icons;

const isIcon = (key: string): key is keyof IconList => {
  return key in Icons;
};

export default Icons;
export { isIcon };
export type { IconList };
