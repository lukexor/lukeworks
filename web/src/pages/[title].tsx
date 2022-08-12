import { NextIcon, PreviousIcon } from "components/icons";
import Layout from "components/layout";
import LinkIcon from "components/linkIcon";
import copy from "data/index.json";
import dayjs from "dayjs";
import fs from "fs/promises";
import { PostEntry } from "models/post";
import { GetStaticPropsContext, GetStaticPropsResult } from "next";
import Head from "next/head";
import Image from "next/image";
import path from "path";
import { ParsedUrlQuery } from "querystring";
import ReactMarkdown from "react-markdown";
import rehypeRaw from "rehype-raw";
import remarkGfm from "remark-gfm";
import s from "./styles/post.module.css";

type PostType = "blog" | "project";
type PostProps = {
  prev: null | PostEntry;
  post: PostEntry & { content: string };
  next: null | PostEntry;
};

interface GetStaticPropsParams extends ParsedUrlQuery {
  title: string;
}
type GetStaticPathsParams = { params: GetStaticPropsParams };

// Get list of posts by type
const getPosts = async (postType: PostType) => {
  const filePath = path.join(process.cwd(), `src/data/${postType}Posts.json`);
  try {
    const jsonData = await fs.readFile(filePath);
    const posts: PostEntry[] = JSON.parse(jsonData.toString());
    return posts;
  } catch (err: unknown) {
    console.error(err);
    return [];
  }
};

export async function getStaticPaths() {
  const blogPosts = await getPosts("blog");
  const projectPosts = await getPosts("project");
  const paths = [...blogPosts, ...projectPosts].reduce<GetStaticPathsParams[]>(
    (paths, currentPost) => {
      if (currentPost.publishedOn) {
        paths.push({ params: { title: currentPost.name } });
      }
      return paths;
    },
    [],
  );
  return {
    paths,
    fallback: false,
  };
}

export async function getStaticProps(
  ctx: GetStaticPropsContext<GetStaticPropsParams>,
): Promise<GetStaticPropsResult<PostProps>> {
  if (!ctx.params) {
    return { notFound: true };
  }

  // Find a single post with previous and next entries
  const findPost = async (title: string, posts: PostEntry[]) => {
    const postIdx = posts.findIndex(
      (post) => post.name.toLowerCase() === title.toLowerCase(),
    );
    if (postIdx > -1) {
      const post = posts[postIdx]!;
      const prev = posts[postIdx + 1];
      const next = posts[postIdx - 1];

      const content = await fs.readFile(
        path.join(process.cwd(), `src/data/posts/${post.name}.md`),
      );
      return {
        prev: prev?.publishedOn ? prev : null,
        post: post?.publishedOn
          ? { ...post, content: content.toString() }
          : null,
        next: next?.publishedOn ? next : null,
      };
    } else {
      return null;
    }
  };

  const blogPosts = await getPosts("blog");
  const projectPosts = await getPosts("project");

  const { title } = ctx.params;
  const match =
    (await findPost(title, blogPosts)) || (await findPost(title, projectPosts));

  if (match && match.post) {
    const { post, prev, next } = match;
    return { props: { post, prev, next } };
  } else {
    return {
      notFound: true,
    };
  }
}

export default function Post({
  post: { title, publishedOn, minutesToRead, image, content, website },
  prev,
  next,
}: PostProps) {
  return (
    <Layout>
      <Head>
        <title>{title}</title>
        <meta property="og:title" content={title} key="title" />
        {image && (
          <meta
            property="og:image"
            content={`${copy.origin}${image.src}`}
            key="image"
          />
        )}
      </Head>
      <section className={s.post}>
        <h1>
          {website ? (
            <a href={website} title={title}>
              {title}
            </a>
          ) : (
            title
          )}
        </h1>
        <p className={s.published}>
          {publishedOn && `${dayjs(publishedOn).format("MMM D, YYYY")}`}
          {publishedOn && minutesToRead > 0 && <>&nbsp;&nbsp; | &nbsp;&nbsp;</>}
          {minutesToRead > 0 && `${minutesToRead} minute read`}
        </p>
        {image && (
          <div>
            <Image
              className={s.postImg}
              src={image.src}
              width={1280}
              height={695}
              layout="responsive"
              priority
              alt={image.alt}
            />
          </div>
        )}
        <section>
          <ReactMarkdown
            skipHtml={false}
            remarkPlugins={[remarkGfm]}
            rehypePlugins={[rehypeRaw]}
          >
            {content}
          </ReactMarkdown>
          <hr />
          <nav className={s.postNav}>
            <LinkIcon
              href={prev?.name ? `/${prev.name}` : undefined}
              title={prev?.title}
              icon={PreviousIcon}
              swapOpacity
            />
            <LinkIcon
              href={next?.name ? `/${next.name}` : undefined}
              title={next?.title}
              icon={NextIcon}
              swapOpacity
            />
          </nav>
        </section>
      </section>
    </Layout>
  );
}
