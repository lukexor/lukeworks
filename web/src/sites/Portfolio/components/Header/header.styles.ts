import styled from "styled-components";

const StyledHeader = styled.header`
  background: ${(props) => props.theme.colors.backgroundLight};
  position: sticky;
  top: 0;
  width: 100%;
  z-index: 10;
`;

const HeaderBar = styled.div`
  display: flex;
  justify-content: space-between;
  flex: 1;
  padding: 10px;
`;

const HeaderIcons = styled.div`
  display: flex;
  justify-content: space-around;
  align-items: middle;
`;

export { HeaderBar, HeaderIcons, StyledHeader };
