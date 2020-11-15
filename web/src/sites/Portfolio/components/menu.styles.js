import styled from "styled-components";

const StyledMenu = styled.span`
  color: ${(props) => props.theme.colors.accentLight};
  cursor: pointer;
  font-size: ${(props) => props.theme.sizes.xlarge};
  margin: auto 0;
`;

export { StyledMenu };
