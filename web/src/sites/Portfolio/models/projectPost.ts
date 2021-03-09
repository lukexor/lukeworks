import { Post } from "./post";

type ProjectPost = Post & {
  website: Maybe<string>;
  startedOn: Maybe<string>;
  completedOn: Maybe<string>;
};

export type { ProjectPost };
