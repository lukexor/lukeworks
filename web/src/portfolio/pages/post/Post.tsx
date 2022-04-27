import "./Post.css";
import dayjs from "dayjs";
import ReactMarkdown from "react-markdown";
import { useParams } from "react-router-dom";
import rehypeRaw from "rehype-raw";
import remarkGfm from "remark-gfm";
import blogPosts from "../../data/blogPosts.json";
import projectPosts from "../../data/projectPosts.json";

type Params = {
  postTitle: string;
};

const Post = () => {
  const { postTitle } = useParams<Params>();

  const post = [...projectPosts, ...blogPosts].find(
    (post) => post.url === postTitle
  );
  if (!post || !post.publishedOn) {
    return null;
  }

  const { title, publishedOn, minutesToRead, image, content } = post;

  return (
    <section className="post">
      <h1>{title}</h1>
      <p className="published">
        {dayjs(publishedOn).format("MMM D, YYYY")} &nbsp;|&nbsp; {minutesToRead}{" "}
        minute read
      </p>
      {image && <img className="post-img" src={image.src} alt={image.alt} />}
      <section>
        <ReactMarkdown
          skipHtml={false}
          remarkPlugins={[remarkGfm]}
          rehypePlugins={[rehypeRaw]}
        >
          {content}
        </ReactMarkdown>
      </section>
    </section>
  );
};

export default Post;
