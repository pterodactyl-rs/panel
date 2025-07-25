import { axiosInstance } from '@/api/axios';

export default async (uuid: string, schedule: number): Promise<void> => {
  return new Promise((resolve, reject) => {
    axiosInstance
      .delete(`/api/client/servers/${uuid}/schedules/${schedule}`)
      .then(() => resolve())
      .catch(reject);
  });
};
