type ProfileInfo = {
  name: string;
  position: string;
  summary: string;
  address: {
    line1: string;
    line2?: string;
    city: string;
    state: string;
    postal: string;
  };
  phone: string;
  email: string;
  website: string;
  github: string;
  linkedIn: string;
};

type Props = {
  profile: ProfileInfo;
};

const Profile = ({ profile }: Props) => <>{profile}</>;

export default Profile;
export type { ProfileInfo };
