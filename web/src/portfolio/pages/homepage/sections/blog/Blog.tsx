import HashAnchor from "portfolio/components/HashAnchor";
import CardGrid from "portfolio/components/post/CardGrid";
import ShowMore from "portfolio/components/post/ShowMore";
import blogPosts from "portfolio/data/blogPosts.json";
import { Post } from "portfolio/models/post";
import { useEffect, useState } from "react";
import routes from "routes.json";

const {
  menu: { blog },
} = routes;

const Blog = () => {
  const [postCount, setPostCount] = useState(9);
  const [totalCount, setTotalCount] = useState(0);
  const [posts, setPosts] = useState<Post[]>([]);

  useEffect(() => {
    const posts = [...blogPosts].filter((post) => post.publishedOn);
    setPosts(posts.slice(0, postCount));
    setTotalCount(posts.length);
  }, [postCount]);

  return (
    <section className="page-section">
      <HashAnchor id={blog.hash} />
      <h2>Blog</h2>
      <CardGrid posts={posts} />
      <ShowMore count={postCount} setCount={setPostCount} total={totalCount} />
    </section>
  );
};

export default Blog;
