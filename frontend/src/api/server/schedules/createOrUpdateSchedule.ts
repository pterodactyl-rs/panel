import { axiosInstance } from '@/api/axios';

interface Data {
  id?: number;
  name: string;
  cron: {
    minute: string;
    hour: string;
    day: string;
    month: string;
    weekday: string;
  };
  onlyWhenOnline: boolean;
  isActive: boolean;
}

export default async (uuid: string, data: Data): Promise<any> => {
  return new Promise((resolve, reject) => {
    axiosInstance
      .post(`/api/client/servers/${uuid}/schedules${data.id ? `/${data.id}` : ''}`, {
        name: data.name,
        minute: data.cron.minute,
        hour: data.cron.hour,
        day_of_month: data.cron.day,
        month: data.cron.month,
        day_of_week: data.cron.weekday,
        only_when_online: data.onlyWhenOnline,
        is_active: data.isActive,
      })
      .then(({ data }) => resolve(null))
      .catch(reject);
  });
};
