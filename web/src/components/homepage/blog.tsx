import HashAnchor from "components/hashAnchor";
import CardGrid, { getCardsPerRow } from "components/post/cardGrid";
import ShowMore from "components/post/showMore";
import blogPosts from "data/blogPosts.json";
import routes from "data/routes.json";
import { PostEntry } from "models/post";
import { useEffect, useState } from "react";
import s from "./homepage.module.css";

export default function Blog() {
  const [postCount, setPostCount] = useState(0);
  const [totalCount, setTotalCount] = useState(0);
  const [posts, setPosts] = useState<PostEntry[]>([]);

  useEffect(() => {
    setPostCount(2 * getCardsPerRow());
  }, []);

  useEffect(() => {
    const posts = [...blogPosts].filter((post) => post.publishedOn);
    setPosts(posts.slice(0, postCount));
    setTotalCount(posts.length);
  }, [postCount]);

  return (
    <section className={s.pageSection}>
      <HashAnchor id={routes.menu.blog.hash} />
      <h2>Blog</h2>
      <CardGrid posts={posts} />
      <ShowMore count={postCount} setCount={setPostCount} total={totalCount} />
    </section>
  );
}
