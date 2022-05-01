type Image = {
  src: string;
  alt: string;
};

type Post = {
  id: number;
  url: string;
  title: string;
  thumbnail?: Image;
  image: null | Image;
  content: string;
  category: string;
  tags: string[];
  minutesToRead: number;
  likes: number;
  publishedOn: null | string;
  createdOn: string;
  updatedOn: string;
  website?: null | string;
  startedOn?: null | string;
  completedOn?: null | string;
};

export type { Image, Post };
