import React from "react";

type HashAnchorProps = {
  id: string;
};

const HashAnchor: React.FC<HashAnchorProps> = ({ id }) => (
  <span id={id.replace("#", "")} className="anchor"></span>
);

export default HashAnchor;
