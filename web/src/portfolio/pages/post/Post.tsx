import "./Post.css";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import dayjs from "dayjs";
import useAuth from "hooks/useAuth";
import NotFound from "NotFound";
import Icons from "portfolio/Icons";
import { Post as PostType } from "portfolio/models/post";
import ReactMarkdown from "react-markdown";
import { Link, useParams } from "react-router-dom";
import rehypeRaw from "rehype-raw";
import remarkGfm from "remark-gfm";
import blogPosts from "../../data/blogPosts.json";
import projectPosts from "../../data/projectPosts.json";

type Params = {
  postLink: string;
};

const getPost = (url = "", posts: PostType[]) => {
  let post;
  let prev;
  let next;
  for (let i = 0; i < posts.length; ++i) {
    if (posts[i]?.url.toLowerCase() === url.toLowerCase()) {
      post = posts[i];
      if (i > 0) {
        next = posts[i - 1];
      }
      if (i < posts.length - 1) {
        prev = posts[i + 1];
      }
      break;
    }
  }
  return [post, prev, next];
};

const Post = () => {
  const { postLink } = useParams<Params>();
  const { isAuthenticated } = useAuth();

  let [post, prev, next] = getPost(postLink, blogPosts);
  if (!post) {
    [post, prev, next] = getPost(postLink, projectPosts);
  }

  if (!post || (!post.publishedOn && !isAuthenticated)) {
    return <NotFound />;
  }

  const { title, publishedOn, minutesToRead, image, content } = post;

  return (
    <section className="post">
      <h1>
        {post.website ? (
          <a href={post.website} title={title}>
            {title}
          </a>
        ) : (
          title
        )}
      </h1>
      <p className="published">
        {publishedOn && `${dayjs(publishedOn).format("MMM D, YYYY")}`}
        {publishedOn && minutesToRead > 0 && <>&nbsp;&nbsp; | &nbsp;&nbsp;</>}
        {minutesToRead > 0 && `${minutesToRead} minute read`}
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
        <hr />
        <nav className="post-nav">
          {prev ? (
            <Link to={`/${prev.url}`} title={prev.title}>
              <FontAwesomeIcon
                className="link-icon"
                title={prev.title}
                icon={Icons.previous}
                swapOpacity
              />
            </Link>
          ) : (
            <FontAwesomeIcon
              className="disabled-icon"
              icon={Icons.previous}
              swapOpacity
            />
          )}
          {next ? (
            <Link to={`/${next.url}`} title={next.title}>
              <FontAwesomeIcon
                className="link-icon"
                title={next.title}
                icon={Icons.next}
                swapOpacity
              />
            </Link>
          ) : (
            <FontAwesomeIcon
              className="disabled-icon"
              icon={Icons.next}
              swapOpacity
            />
          )}
        </nav>
      </section>
    </section>
  );
};

export default Post;
