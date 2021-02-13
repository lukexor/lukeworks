import React from "react";

type ProfileProps = {
  name: string;
  position: string;
  summary: string;
  address: {
    line1: string;
    line2?: string;
    city: string;
    state: string;
    postal: string;
  },
  phone: string;
  email: string;
  website: string;
  github: string;
  linkedIn: string;
};

const Profile: React.FC<ProfileProps> = () => <></>;

export default Profile;
