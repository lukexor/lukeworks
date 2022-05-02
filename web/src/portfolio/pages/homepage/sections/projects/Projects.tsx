import HashAnchor from "portfolio/components/HashAnchor";
import CardGrid from "portfolio/components/post/CardGrid";
import ShowMore from "portfolio/components/post/ShowMore";
import projectPosts from "portfolio/data/projectPosts.json";
import { Post } from "portfolio/models/post";
import { useEffect, useState } from "react";
import routes from "routes.json";

const {
  menu: { projects },
} = routes;

const Projects = () => {
  const [postCount, setPostCount] = useState(6);
  const [totalCount, setTotalCount] = useState(0);
  const [posts, setPosts] = useState<Post[]>([]);

  useEffect(() => {
    const posts = [...projectPosts].filter((post) => post.publishedOn);
    setPosts(posts.slice(0, postCount));
    setTotalCount(posts.length);
  }, [postCount]);

  return (
    <section className="page-section">
      <HashAnchor id={projects.hash} />
      <h2>Projects</h2>
      <CardGrid posts={posts} />
      <ShowMore count={postCount} setCount={setPostCount} total={totalCount} />
    </section>
  );
};

export default Projects;
