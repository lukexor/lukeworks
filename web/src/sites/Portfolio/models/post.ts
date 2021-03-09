type Image = {
  src: string;
  alt: string;
};

type Post = {
  id: number;
  url: string;
  title: string;
  thumbnail: Image;
  image: Maybe<Image>;
  content: string;
  category: string;
  tags: string[];
  minutesToRead: number;
  likes: number;
  publishedOn: Maybe<string>;
  createdOn: string;
  updatedOn: string;
};

export type { Image, Post };
