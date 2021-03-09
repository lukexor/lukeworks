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
  type JSONPrimitive = string | number | boolean | null;
  type JSONValue = JSONPrimitive | JSONArray | JSONObject;
  interface JSONObject {
    [name: string]: JSONValue;
  }
  interface JSONArray {
    [index: number]: JSONValue;
  }
}

// declare module "*.json" {
//   const value: JSONObject | JSONArray;
//   export = value;
// }

declare module "styled-components" {
  export interface DefaultTheme {
    breakpoints: {
      desktopLarge: string;
      desktopSmall: string;
      tablet: string;
      mobile: string;
    };
    colors: {
      accentDark: string;
      accentLight: string;
      background: string;
      backgroundLight: string;
      primary: string;
      secondary: string;
    };
    fontSerif: string;
    fontSans: string;
    rootFontSize: string;
    boxShadows: {
      small: string;
      med: string;
      large: string;
    };
    sizes: {
      xsmall: string;
      small: string;
      medSmall: string;
      med: string;
      medLarge: string;
      large: string;
      xlarge: string;
      xxlarge: string;
      xxxlarge: string;
    };
  }
}
