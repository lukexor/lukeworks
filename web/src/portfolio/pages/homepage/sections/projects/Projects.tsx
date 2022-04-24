// import { Link } from "react-router-dom";
// import HashAnchor from "portfolio/components/HashAnchor";
// import projectPosts from "portfolio/data/projectPosts.json";
// import { ProjectPost } from "portfolio/models/projectPost";
// import routes from "portfolio/routes.json";

// type ProjectPostCardProps = {
//   post: ProjectPost;
// };

// const ProjectPostCard: React.FC<ProjectPostCardProps> = ({ post }) => {
//   const { publishedOn, url, thumbnail, title } = post;

//   return publishedOn ? (
//     <StyledProjectPostCard>
//       <Link to={url}>
//         {thumbnail.src ? (
//           <ProjectThumbnail src={thumbnail.src} alt={thumbnail.alt} />
//         ) : (
//           <p>{title}</p>
//         )}
//       </Link>
//     </StyledProjectPostCard>
//   ) : null;
// };

// const Projects: React.FC = () => (
//   <StyledProjects>
//     <HashAnchor id={routes.projects.slice(1)} />
//     <h2>Projects</h2>
//     <ProjectPosts>
//       {projectPosts.map((post) => (
//         <ProjectPostCard key={post.id} post={post} />
//       ))}
//     </ProjectPosts>
//   </StyledProjects>
// );

const Project = () => {
  return <></>;
};

export default Project;
