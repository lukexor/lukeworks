import type p5 from "p5";

declare global {
  type JSONPrimitive = string | number | boolean | null;
  type JSONValue = JSONPrimitive | JSONArray | JSONObject;
  interface JSONObject {
    [name: string]: JSONValue;
  }
  interface JSONArray {
    [index: number]: JSONValue;
  }

  interface Window {
    p5: typeof p5;
  }
}
