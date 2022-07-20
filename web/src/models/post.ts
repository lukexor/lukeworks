export type PostImage = {
  src: string;
  alt: string;
};

export type PostEntry = {
  id: number;
  url: string;
  title: string;
  thumbnail?: PostImage;
  image: null | PostImage;
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
