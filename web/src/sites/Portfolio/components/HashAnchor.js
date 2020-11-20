import PropTypes from "prop-types";
import React from "react";

const HashAnchor = ({ id }) => (
  <span id={id.replace("#", "")} className="anchor"></span>
);

HashAnchor.propTypes = {
  id: PropTypes.string.isRequired,
};

export default HashAnchor;
