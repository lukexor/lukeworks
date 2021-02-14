import copy from "./data/copy.json";

type Link = {
  name: string;
  path: string;
  title: string;
};

type MenuMap = {
  [key: string]: Link;
};

const MenuLinks = copy.Menu.links.reduce((map, link) => {
  map[link.name] = link;
  return map;
}, {} as MenuMap);

const routes = {
  ...MenuLinks,
  post: {
    path: "/:title",
    title: "Post",
  },
  legacy: {
    articles: "/articles",
    articleByTitle: "/articles/:year?/:month?/:title?",
    articlesByCategory: "/articles/category/:category",
    articlesByTag: "/articles/tag/:tag",
    articleSearch: "/articles/search/:query",
    projectByTitle: "/projects/:year?/:month?/:title?",
    projectsByTag: "/projects/tag/:tag",
    rss: "/feed",
  },
};

export default routes;
