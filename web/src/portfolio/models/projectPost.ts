import { Post } from "./post";

type ProjectPost = Post & {
  website: null | string;
  startedOn: null | string;
  completedOn: null | string;
};

export type { ProjectPost };
