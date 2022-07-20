import s from "./loading.module.css";

export default function Loading() {
  return (
    <div className={s.wrapper}>
      <div className={s.loading}>
        <div></div>
        <div></div>
        <div></div>
      </div>
    </div>
  );
}
