const scrollWithHeaderOffset = (el) => {
  const yCoordinate = el.getBoundingClientRect().top + window.pageYOffset;
  let yOffset = 0;
  const header = document.getElementsByTagName("header")[0];
  if (header) {
    const headerHeight = header.getBoundingClientRect().height;
    yOffset = -1 * headerHeight;
  }
  window.scrollTo({ top: yCoordinate + yOffset, behavior: "smooth" });
};

export { scrollWithHeaderOffset };
