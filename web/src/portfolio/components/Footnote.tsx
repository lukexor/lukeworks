// type FootnoteReferenceProps = {
//   identifier: string;
//   label: string;
// };

// type FootnoteDefinitionProps = FootnoteReferenceProps & {
//   children: React.ReactElement;
// };

// const FootnoteAnchor = styled.span`
//   top: -5em;
//   position: relative;
//   display: inline-block;
// `;

// const StyledSup = styled.sup`
//   margin-left: 2px;
// `;

// const FootnoteReference = ({
//   identifier,
//   label,
// }: FootnoteReferenceProps): React.ReactElement => (
//   <StyledSup>
//     <FootnoteAnchor id={"ref-" + identifier} />
//     <a href={"#def-" + identifier}>{label}</a>
//   </StyledSup>
// );

// const StyledFootnoteDefWrapper = styled.div`
//   font-size: ${({ theme }) => theme.sizes.medSmall};
// `;
// const StyledFootnoteDef = styled.div`
//   display: inline;
//   p:first-of-type {
//     display: inline;
//   }
// `;

// const FootnoteDefinition = ({
//   identifier,
//   label,
//   children,
// }: FootnoteDefinitionProps): React.ReactElement => (
//   <StyledFootnoteDefWrapper>
//     <FootnoteAnchor id={"def-" + identifier} />
//     <a href={"#ref-" + identifier}>{label}</a>:{" "}
//     <StyledFootnoteDef>{children}</StyledFootnoteDef>
//   </StyledFootnoteDefWrapper>
// );

const FootnoteReference = () => {
  return <></>;
};
const FootnoteDefinition = () => {
  return <></>;
};

const renderers = {
  footnoteReference: FootnoteReference,
  footnoteDefinition: FootnoteDefinition,
};

export { FootnoteDefinition, FootnoteReference, renderers };
