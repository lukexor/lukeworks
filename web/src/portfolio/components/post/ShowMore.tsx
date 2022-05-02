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
    const itemsPerRow = Math.floor(
      pageWidth /
        (document.getElementsByClassName("card")[0]?.clientWidth ?? pageWidth)
    );
    setCount((count: number) => count + itemsPerRow);
    if (actionsRef.current) {
      window.scrollTo({
        top: actionsRef.current.offsetTop - 200,
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
