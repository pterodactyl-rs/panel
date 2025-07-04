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
    <div className="bg-gray-700 p-4 rounded flex gap-4">
      <FontAwesomeIcon className="text-gray-100 bg-gray-600 p-4 rounded-lg" size={'xl'} icon={icon} />
      <div className="flex flex-col">
        <span className="text-sm text-gray-400 font-bold">{label}</span>
        <span className="text-lg font-bold">
          {value} {limit && <span className="text-sm text-gray-400">/ {limit}</span>}
        </span>
      </div>
    </div>
  );
}

export default () => {
  const server = useServerStore(state => state.data);
  const stats = useServerStore(state => state.stats);

  const diskLimit = server.limits.disk !== 0 ? bytesToString(mbToBytes(server.limits.disk)) : 'Unlimited';
  const memoryLimit = server.limits.memory !== 0 ? bytesToString(mbToBytes(server.limits.memory)) : 'Unlimited';
  const cpuLimit = server.limits.cpu !== 0 ? server.limits.cpu + '%' : 'Unlimited';

  return (
    <div className="col-span-1 grid gap-4">
      <StatCard icon={faEthernet} label="Address" value={formatAllocation(getPrimaryAllocation(server.allocations))} />
      <StatCard
        icon={faClock}
        label="Uptime"
        value={stats.state === 'offline' ? 'Offline' : formatMiliseconds(stats.uptime || 0)}
      />
      <StatCard
        icon={faMicrochip}
        label="CPU Load"
        value={stats.state === 'offline' ? 'Offline' : `${stats.cpu.toFixed(2)}%`}
        limit={stats.state === 'offline' ? null : cpuLimit}
      />
      <StatCard
        icon={faMemory}
        label="Memory Load"
        value={stats.state === 'offline' ? 'Offline' : bytesToString(stats.memory)}
        limit={stats.state === 'offline' ? null : memoryLimit}
      />
      <StatCard icon={faHardDrive} label="Disk Usage" value={bytesToString(stats.disk)} limit={diskLimit} />
      <StatCard
        icon={faCloudDownload}
        label="Network (In)"
        value={stats.state === 'offline' ? 'Offline' : bytesToString(stats.rx)}
      />
      <StatCard
        icon={faCloudUpload}
        label="Network (Out)"
        value={stats.state === 'offline' ? 'Offline' : bytesToString(stats.tx)}
      />
    </div>
  );
};
