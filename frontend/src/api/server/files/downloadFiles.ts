import { axiosInstance } from '@/api/axios';
import { createSearchParams } from 'react-router';

export default async (uuid: string, root: string, paths: string[], isDirectory: boolean): Promise<{ url: string }> => {
  return new Promise((resolve, reject) => {
    axiosInstance
      .get(
        `/api/client/servers/${uuid}/files/download?${createSearchParams({
          root,
          files: paths,
          directory: isDirectory.toString(),
        })}`,
      )
      .then(({ data }) => resolve(data))
      .catch(reject);
  });
};
