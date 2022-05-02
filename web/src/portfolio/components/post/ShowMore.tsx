import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import Icons from "portfolio/Icons";
import { Dispatch, SetStateAction, useRef } from "react";

const maxCount = 90;

type Props = {
  count: number;
  setCount: Dispatch<SetStateAction<number>>;
  total: number;
};

const ShowMore = ({ count, setCount, total }: Props) => {
  const actionsRef = useRef<HTMLDivElement>(null);

  if (count >= Math.min(total, maxCount)) {
    return null;
  }

  const showMore = () => {
    const pageWidth = document.body.clientWidth;
    const card = document.getElementsByClassName("card")[0];
    const cardWidth = card?.clientWidth ?? pageWidth;
    const cardHeight = card?.clientHeight ?? 200;
    const itemsPerRow = Math.floor(pageWidth / cardWidth);
    setCount((count: number) => count + itemsPerRow);
    if (actionsRef.current) {
      window.scrollTo({
        top: actionsRef.current.offsetTop - cardHeight,
        behavior: "smooth",
      });
    }
  };

  return (
    <div ref={actionsRef} className="page-actions">
      <FontAwesomeIcon
        className="link-icon"
        icon={Icons.more}
        onClick={showMore}
      />
    </div>
  );
};

export default ShowMore;
