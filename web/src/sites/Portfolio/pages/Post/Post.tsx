import React from "react";
import ReactMarkdown from "react-markdown";
import { useParams } from "react-router-dom";
import footnotes from "remark-footnotes";
import gfm from "remark-gfm";
import styled from "styled-components";
import { renderers as footnoteRenderers } from "../../components/Footnote";
import blogPosts from "../../data/blogPosts.json";
import projectPosts from "../../data/projectPosts.json";

type Params = {
  postTitle: string;
};

const StyledPost = styled.section`
  margin: auto;
  margin-bottom: 100px;
  max-width: 70ch;
  display: flex;
  flex-direction: column;
  justify-content: center;

  h2 {
    font-size: 2rem;
  }

  pre {
    padding: 10px;
    color: ${({ theme }) => theme.colors.background};
    background-color: ${({ theme }) => theme.colors.secondary};
  }

  small {
    color: ${({ theme }) => theme.colors.accentDark};
  }
`;

const PostTitle = styled.h1`
  font-size: 2.5rem;
  text-align: left;
  margin-bottom: 0.2rem;
`;

const PostPublished = styled.p`
  margin: 0;
  margin-bottom: 2rem;
  font-size: 0.8rem;
  color: ${({ theme }) => theme.colors.accentDark};
`;

const PostImage = styled.img`
  width: 100%;
`;

const PostContent = styled.section``;

const markdownPlugins = [gfm, footnotes];

// TODO: Post
const Post: React.FC = () => {
  const { postTitle } = useParams<Params>();

  const post = [...projectPosts, ...blogPosts].find(
    (post) => post.url === postTitle
  );
  if (!post || !post.publishedOn) {
    return null;
  }

  const { title, publishedOn, minutesToRead, image, content } = post;
  return (
    <StyledPost>
      <PostTitle>{title}</PostTitle>
      <PostPublished>
        {new Date(publishedOn).toLocaleString()} &nbsp;|&nbsp; {minutesToRead}{" "}
        minute read
      </PostPublished>
      {image && <PostImage src={image.src} alt={image.alt} />}
      <PostContent>
        <ReactMarkdown
          allowDangerousHtml
          plugins={markdownPlugins}
          renderers={footnoteRenderers}
        >
          {content}
        </ReactMarkdown>
      </PostContent>
    </StyledPost>
  );
};

export default Post;
