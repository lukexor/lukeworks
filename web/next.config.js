/* @type {import('next').NextConfig} */
module.exports = {
  reactStrictMode: true,
  poweredByHeader: false,
  swcMinify: true,
  async redirects() {
    return [
      {
        source: "/about",
        destination: "/#about",
        permanent: true,
      },
      {
        source: "/articles",
        destination: "/#blog",
        permanent: true,
      },
      {
        source: "/articles/2019/09/rustynes_part_1",
        destination: "/tetanes-part-1",
        permanent: true,
      },
      {
        source: "/articles/2020/01/rustynes_part_2",
        destination: "/tetanes-part-2",
        permanent: true,
      },
      {
        source: "/projects/2019/08/rustynes",
        destination: "/tetanes",
        permanent: true,
      },
      {
        source: "/articles/[year]/[month]/[title]",
        destination: "/[title]",
        permanent: true,
      },
      {
        source: "/articles/category/[category]",
        destination: "/category/[category]",
        permanent: true,
      },
      {
        source: "/[section]/tag/[tag]",
        destination: "/tag/[tag]",
        permanent: true,
      },
      {
        source: "/contact",
        destination: "/#contact",
        permanent: true,
      },
      {
        source: "/feed",
        destination: "/rss",
        permanent: true,
      },
      {
        source: "/projects",
        destination: "/#projects",
        permanent: true,
      },
    ];
  },
};
