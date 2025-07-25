import Spinner from '@/elements/Spinner';
import { formatAllocation, getPrimaryAllocation } from '@/lib/server';
import { bytesToString, mbToBytes } from '@/lib/size';
import { formatMiliseconds } from '@/lib/time';
import { useServerStore } from '@/stores/server';
import {
  faClock,
  faCloudDownload,
  faCloudUpload,
  faEthernet,
  faHardDrive,
  faMemory,
  faMicrochip,
  IconDefinition,
} from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';

function StatCard({
  icon,
  label,
  value,
  limit,
}: {
  icon: IconDefinition;
  label: string;
  value: string;
  limit?: string;
}) {
  return (
    <div className={'bg-gray-700 p-4 rounded flex gap-4'}>
      <FontAwesomeIcon className={'text-gray-100 bg-gray-600 p-4 rounded-lg'} size={'xl'} icon={icon} />
      <div className={'flex flex-col'}>
        <span className={'text-sm text-gray-400 font-bold'}>{label}</span>
        <span className={'text-lg font-bold'}>
          {value} {limit && <span className={'text-sm text-gray-400'}>/ {limit}</span>}
        </span>
      </div>
    </div>
  );
}

export default () => {
  const server = useServerStore((state) => state.server);
  const stats = useServerStore((state) => state.stats);
  const state = useServerStore((state) => state.state);

  const diskLimit = server.limits.disk !== 0 ? bytesToString(mbToBytes(server.limits.disk)) : 'Unlimited';
  const memoryLimit = server.limits.memory !== 0 ? bytesToString(mbToBytes(server.limits.memory)) : 'Unlimited';
  const cpuLimit = server.limits.cpu !== 0 ? server.limits.cpu + '%' : 'Unlimited';

  return stats ? (
    <div className={'col-span-1 grid gap-4'}>
      <StatCard
        icon={faEthernet}
        label={'Address'}
        value={server.allocation ? formatAllocation(server.allocation) : 'N/A'}
      />
      <StatCard
        icon={faClock}
        label={'Uptime'}
        value={state === 'offline' ? 'Offline' : formatMiliseconds(stats.uptime || 0)}
      />
      <StatCard
        icon={faMicrochip}
        label={'CPU Load'}
        value={state === 'offline' ? 'Offline' : `${stats.cpuAbsolute.toFixed(2)}%`}
        limit={state === 'offline' ? null : cpuLimit}
      />
      <StatCard
        icon={faMemory}
        label={'Memory Load'}
        value={state === 'offline' ? 'Offline' : bytesToString(stats.memoryBytes)}
        limit={state === 'offline' ? null : memoryLimit}
      />
      <StatCard icon={faHardDrive} label={'Disk Usage'} value={bytesToString(stats.diskBytes)} limit={diskLimit} />
      <StatCard
        icon={faCloudDownload}
        label={'Network (In)'}
        value={state === 'offline' ? 'Offline' : bytesToString(stats.network.rxBytes)}
      />
      <StatCard
        icon={faCloudUpload}
        label={'Network (Out)'}
        value={state === 'offline' ? 'Offline' : bytesToString(stats.network.txBytes)}
      />
    </div>
  ) : (
    <Spinner.Centered />
  );
};
