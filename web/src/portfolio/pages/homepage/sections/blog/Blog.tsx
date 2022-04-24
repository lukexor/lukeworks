// import { Link } from "react-router-dom";
// import HashAnchor from "portfolio/components/HashAnchor";
// import blogPosts from "portfolio/data/blogPosts.json";
// import { Post } from "portfolio/models/post";
// import routes from "portfolio/routes.json";
// import {
//   BlogPosts,
//   BlogThumbnail,
//   StyledBlog,
//   StyledBlogPostCard,
// } from "./blog.styles";

// type BlogPostCardProps = {
//   post: Post;
// };

// // TODO: Make BlogThumbnail a component with an image placeholder
// const BlogPostCard = ({ post }) => {
//   const { url, thumbnail, title } = post;
//   return post.publishedOn ? (
//     <StyledBlogPostCard>
//       <Link to={url}>
//         {thumbnail && <BlogThumbnail src={thumbnail.src} alt={thumbnail.alt} />}
//         <h3>{title}</h3>
//       </Link>
//     </StyledBlogPostCard>
//   ) : null;
// };

// const Blog = () => {
//   return (
//     <StyledBlog>
//       <HashAnchor id={routes.blog.slice(1)} />
//       <h2>Blog</h2>
//       <BlogPosts>
//         {blogPosts.map((post) => (
//           <BlogPostCard key={post.id} post={post} />
//         ))}
//       </BlogPosts>
//     </StyledBlog>
//   );
// };
const Blog = () => {
  return <></>;
};

export default Blog;
