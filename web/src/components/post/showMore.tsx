import { MoreIcon } from "components/icons";
import LinkIcon from "components/linkIcon";
import { Dispatch, SetStateAction, useRef } from "react";
import { getCardsPerRow } from "./cardGrid";
import s from "./showMore.module.css";

export type ShowMoreProps = {
  count: number;
  setCount: Dispatch<SetStateAction<number>>;
  total: number;
};

export default function ShowMore({ count, setCount, total }: ShowMoreProps) {
  const actionsRef = useRef<HTMLDivElement>(null);

  if (count >= total) {
    return null;
  }

  const showMore = () => {
    const cardsPerRow = getCardsPerRow();
    const cardHeight =
      document.querySelector("[data-type=card]")?.clientHeight ?? 180;
    setCount((count: number) => count + cardsPerRow);
    if (actionsRef.current) {
      window.scrollTo({
        top: actionsRef.current.offsetTop - cardHeight,
        behavior: "smooth",
      });
    }
  };

  return (
    <div ref={actionsRef} className={s.pageActions}>
      <LinkIcon icon={MoreIcon} onClick={showMore} />
    </div>
  );
}
