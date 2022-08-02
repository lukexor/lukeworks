export type PostImage = {
  src: string;
  alt: string;
};

export type PostEntry = {
  id: number;
  name: string;
  title: string;
  thumbnail?: PostImage;
  image: null | PostImage;
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
