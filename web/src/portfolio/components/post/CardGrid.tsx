import classnames from "classnames";
import useOnScreen from "hooks/useOnScreen";
import { Image, Post } from "portfolio/models/post";
import { createRef, forwardRef } from "react";
import { Link } from "react-router-dom";
import { CSSTransition, TransitionGroup } from "react-transition-group";

type CardProps = {
  url: string;
  thumbnail?: Image;
  title: string;
};

type CardGridProps = {
  posts: Post[];
};

const Card = forwardRef<HTMLElement, CardProps>(
  ({ title, thumbnail, url }, ref) => {
    const [imgRef, isImgVisible] = useOnScreen<HTMLDivElement>();
    return (
      <article ref={ref}>
        <Link to={url}>
          <div className="card">
            <div ref={imgRef}>
              {isImgVisible && thumbnail ? (
                <img className="blur" src={thumbnail.src} alt={thumbnail.alt} />
              ) : (
                <div className="card-placeholder" />
              )}
            </div>
            <div
              className={classnames({ title: true, "slide-up": !!thumbnail })}
            >
              {title}
            </div>
          </div>
        </Link>
      </article>
    );
  }
);

Card.displayName = "Card";

const CardGrid = ({ posts }: CardGridProps) => {
  return (
    <section className="lazy">
      <TransitionGroup component="span" className="card-grid">
        {posts.map(({ id, title, thumbnail, url }) => {
          const postRef = createRef<HTMLElement>();
          return (
            <CSSTransition
              key={id}
              nodeRef={postRef}
              timeout={500}
              classNames="post"
            >
              <Card
                key={id}
                ref={postRef}
                title={title}
                thumbnail={thumbnail}
                url={url}
              />
            </CSSTransition>
          );
        })}
      </TransitionGroup>
    </section>
  );
};

export default CardGrid;
