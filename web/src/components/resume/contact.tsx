export type ContactInfo = {
  email: string;
  website: string;
  github: string;
  linkedIn: string;
};

export type ContactProps = {
  contact: ContactInfo;
};

export default function Contact({ contact }: ContactProps) {
  return (
    <section>
      <h3>Contact Info</h3>
      <h4>Email</h4>
      <a href={contact.email}>{contact.email}</a>
      <h4>Website</h4>
      <a href={`https://${contact.website}`}>{contact.website}</a>
      <h4>GitHub</h4>
      <a href={`https://${contact.github}`}>{contact.github}</a>
      <h4>LinkedIn</h4>
      <a href={`https://${contact.linkedIn}`}>{contact.linkedIn}</a>
    </section>
  );
}
