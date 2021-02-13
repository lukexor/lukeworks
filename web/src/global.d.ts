// https://styled-components.com/docs/api#usage-with-typescript
import p5 from "p5";
import * as _p5Global from "p5/global";
import {} from "styled-components/cssprop";

export = p5;
export as namespace p5;
declare global {
  interface Window {
    p5: typeof p5;
  }

  type Maybe<T> = null | undefined | T;
}
