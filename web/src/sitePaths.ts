const SitePaths = {
  about: "#about",
  blog: "#blog",
  contact: "#contact",
  projects: "#projects",
  home: "#",
  post: "/:title",
  resume: "/resume",
  admin: "/admin",
  login: "/login",
  tetanes: "/tetanes",
};

const LegacyPaths = {
  articles: "/articles",
  articleByTitle: "/articles/:year?/:month?/:title?",
  articlesByCategory: "/articles/category/:category",
  articlesByTag: "/articles/tag/:tag",
  articleSearch: "/articles/search/:query",
  projectByTitle: "/projects/:year?/:month?/:title?",
  projectsByTag: "/projects/tag/:tag",
  rss: "/feed",
};

export default SitePaths;
export { LegacyPaths };
