import styled from "styled-components";

const StyledHeader = styled.header`
  display: flex;
  justify-content: space-between;
  background: ${(props) => props.theme.colors.background};
  mix-blend-mode: darken;
  padding: ${(props) => `${props.theme.sizes.small} ${props.theme.sizes.med}`};
  position: sticky;
  top: 0;
  height: 5vh;
`;

export { StyledHeader };
