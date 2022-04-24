import { useEffect } from "react";

type Props = {
  title?: string;
  description?: string;
};

const useMetaTag = ({ title, description }: Props) => {
  useEffect(() => {
    if (title) {
      document.title = title;
    }
    if (description) {
      const meta_tag: null | HTMLMetaElement = document.head.querySelector(
        "meta[name='description']"
      );
      if (meta_tag) {
        meta_tag.content = description;
      }
    }
  }, []);
};

export default useMetaTag;
