import Image from "next/image";
import s from "pages/styles/resume.module.css";

export default function Ribbon() {
  return (
    <div className={s.ribbon}>
      <Image src="/images/resume/ribbon.png" width={35} height={35} alt="" />
    </div>
  );
}
