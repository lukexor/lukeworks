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

export const SearchIcon = faSearch;
export const ClearIcon = faTimes;
export const MenuIcon = faBars;
export const BackToTopIcon = faAngleDoubleUp;
export const GithubIcon = faGithub;
export const LinkedInIcon = faLinkedin;
export const RssIcon = faRssSquare;
export const EmailIcon = faEnvelope;
export const ExploreIcon = faCircleChevronDown;
export const PreviousIcon = faCircleChevronLeft;
export const NextIcon = faCircleChevronRight;
export const MoreIcon = faAngleDown;

export const SocialIcons = {
  github: GithubIcon,
  linkedIn: LinkedInIcon,
  rss: RssIcon,
  email: EmailIcon,
};

export const isSocialIcon = (
  icon: PropertyKey,
): icon is keyof typeof SocialIcons => {
  return icon in SocialIcons;
};
