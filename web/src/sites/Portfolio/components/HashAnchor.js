import PropTypes from "prop-types";
import React from "react";

const HashAnchor = ({ id }) => (
  <a id={id.replace("#", "")} className="anchor"></a>
);

HashAnchor.propTypes = {
  id: PropTypes.string.isRequired,
};

export default HashAnchor;
