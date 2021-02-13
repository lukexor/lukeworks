import copy from "./data/copy.json";

const routes = {
  ...copy.Menu.links,
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
