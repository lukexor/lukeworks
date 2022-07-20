import clsx from "clsx";
import { PostEntry, PostImage } from "models/post";
import Image from "next/image";
import Link from "next/link";
import { createRef, ForwardedRef, forwardRef, useRef } from "react";
import { CSSTransition, TransitionGroup } from "react-transition-group";
import s from "./cardGrid.module.css";

type CardProps = {
  url: string;
  thumbnail?: PostImage;
  title: string;
};

type CardGridProps = {
  posts: PostEntry[];
};

export const getCardsPerRow = (): number => {
  const sectionWidth =
    document.querySelector("[data-type=grid]")?.clientWidth ?? 1;
  const cardWidth =
    document.querySelector("[data-type=card]")?.clientWidth ?? 300;
  return Math.max(1, Math.floor(sectionWidth / cardWidth));
};

const Card = forwardRef(function Card(
  { title, thumbnail, url }: CardProps,
  ref: ForwardedRef<HTMLElement>,
) {
  const imgRef = useRef<HTMLDivElement>(null);
  return (
    <article ref={ref}>
      <Link href={url}>
        <a>
          <div className={s.card} data-type="card">
            <div ref={imgRef}>
              {thumbnail ? (
                <Image
                  className={s.blur}
                  src={thumbnail.src}
                  width={1000}
                  height={600}
                  alt={thumbnail.alt}
                  layout="responsive"
                />
              ) : (
                <div className={s.cardPlaceholder} />
              )}
            </div>
            <div className={clsx(s.title, !!thumbnail && s.slideUp)}>
              {title}
            </div>
          </div>
        </a>
      </Link>
    </article>
  );
});

export default function CardGrid({ posts }: CardGridProps) {
  return (
    <section>
      <TransitionGroup component="span" className={s.cardGrid} data-type="grid">
        {posts.map(({ id, title, thumbnail, url }) => {
          const postRef = createRef<HTMLElement>();
          return (
            <CSSTransition
              key={id}
              nodeRef={postRef}
              timeout={500}
              classNames={{
                enter: s.postEnter,
                enterActive: s.postEnterActive,
                exit: s.postExit,
                exitActive: s.postExitActive,
              }}
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
}
