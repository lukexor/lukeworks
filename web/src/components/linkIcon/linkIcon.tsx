import type { IconProp } from "@fortawesome/fontawesome-svg-core";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import clsx from "clsx";
import type { LinkProps } from "next/link";
import Link from "next/link";
import s from "./linkIcon.module.css";

export type Props = Partial<LinkProps> & {
  title?: string;
  icon: IconProp;
  swapOpacity?: boolean;
  className?: string;
};

export default function LinkIcon({
  href,
  onClick,
  title,
  icon,
  swapOpacity = false,
  className,
  ...props
}: Props) {
  const iconProps = {
    className: clsx(href || onClick ? s.linkIcon : s.disabledIcon, className),
    titleId: title, // to avoid FontAwesome generating a unique one causing re-hydration errors
    title,
    icon,
    swapOpacity,
  };

  if (href) {
    return (
      <Link href={href} onClick={onClick} {...props}>
        <FontAwesomeIcon {...iconProps} />
      </Link>
    );
  } else {
    return (
      <span onClick={onClick}>
        <FontAwesomeIcon {...iconProps} />
      </span>
    );
  }
}
