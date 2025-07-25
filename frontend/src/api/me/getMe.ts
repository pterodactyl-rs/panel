import { axiosInstance } from '@/api/axios';

export default async (): Promise<User> => {
  return new Promise((resolve, reject) => {
    axiosInstance
      .get('/api/client/account')
      .then(({ data }) => resolve(data.user))
      .catch(reject);
  });
};
