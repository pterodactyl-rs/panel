import { FractalResponseData, FractalResponseList } from './axios';

export const rawDataToServerAllocation = (data: FractalResponseData): Allocation => ({
  id: data.attributes.id,
  ip: data.attributes.ip,
  alias: data.attributes.ip_alias,
  port: data.attributes.port,
  notes: data.attributes.notes,
  isDefault: data.attributes.is_default,
});

export const rawDataToFileObject = (data: FractalResponseData): FileObject => ({
  key: `${data.attributes.is_file ? 'file' : 'dir'}_${data.attributes.name}`,
  name: data.attributes.name,
  mode: data.attributes.mode,
  modeBits: data.attributes.mode_bits,
  size: Number(data.attributes.size),
  isFile: data.attributes.is_file,
  isSymlink: data.attributes.is_symlink,
  mimetype: data.attributes.mimetype,
  createdAt: new Date(data.attributes.created_at),
  modifiedAt: new Date(data.attributes.modified_at),

  isArchiveType: function () {
    return (
      this.isFile &&
      [
        'application/vnd.rar', // .rar
        'application/x-rar-compressed', // .rar (2)
        'application/x-tar', // .tar
        'application/x-br', // .tar.br
        'application/x-bzip2', // .tar.bz2, .bz2
        'application/gzip', // .tar.gz, .gz
        'application/x-gzip',
        'application/x-lzip', // .tar.lz4, .lz4 (not sure if this mime type is correct)
        'application/x-sz', // .tar.sz, .sz (not sure if this mime type is correct)
        'application/x-xz', // .tar.xz, .xz
        'application/zstd', // .tar.zst, .zst
        'application/zip', // .zip
        'application/x-7z-compressed', // .7z
      ].indexOf(this.mimetype) >= 0
    );
  },

  isDirectory: function () {
    return this.mimetype === 'inode/directory';
  },

  isEditable: function () {
    if (this.isArchiveType() || !this.isFile) return false;

    const matches = ['application/jar', 'application/octet-stream', 'inode/directory', /^image\/(?!svg\+xml)/];

    return matches.every(m => !this.mimetype.match(m));
  },
});

export const rawDataToServerBackup = ({ attributes }: FractalResponseData): ServerBackup => ({
  uuid: attributes.uuid,
  isSuccessful: attributes.is_successful,
  isLocked: attributes.is_locked,
  name: attributes.name,
  ignoredFiles: attributes.ignored_files,
  checksum: attributes.checksum,
  bytes: attributes.bytes,
  createdAt: new Date(attributes.created_at),
  completedAt: attributes.completed_at ? new Date(attributes.completed_at) : null,
});

export const rawDataToServerEggVariable = ({ attributes }: FractalResponseData): ServerEggVariable => ({
  name: attributes.name,
  description: attributes.description,
  envVariable: attributes.env_variable,
  defaultValue: attributes.default_value,
  serverValue: attributes.server_value,
  isEditable: attributes.is_editable,
  rules: attributes.rules.split('|'),
});

export const rawDataToServerObject = ({ attributes: data }: FractalResponseData): Server => ({
  id: data.identifier,
  internalId: data.internal_id,
  uuid: data.uuid,
  name: data.name,
  node: data.node,
  isNodeUnderMaintenance: data.is_node_under_maintenance,
  status: data.status,
  invocation: data.invocation,
  dockerImage: data.docker_image,
  sftpDetails: {
    ip: data.sftp_details.ip,
    port: data.sftp_details.port,
  },
  description: data.description ? (data.description.length > 0 ? data.description : null) : null,
  limits: { ...data.limits },
  eggFeatures: data.egg_features || [],
  featureLimits: { ...data.feature_limits },
  isTransferring: data.is_transferring,
  variables: ((data.relationships?.variables as FractalResponseList | undefined)?.data || []).map(
    rawDataToServerEggVariable,
  ),
  allocations: ((data.relationships?.allocations as FractalResponseList | undefined)?.data || []).map(
    rawDataToServerAllocation,
  ),
});

export const rawDataToServerStats = ({ attributes }: FractalResponseData): ServerStats => ({
  status: attributes.current_state,
  isSuspended: attributes.is_suspended,
  memoryUsageInBytes: attributes.resources.memory_bytes,
  cpuUsagePercent: attributes.resources.cpu_absolute,
  diskUsageInBytes: attributes.resources.disk_bytes,
  networkRxInBytes: attributes.resources.network_rx_bytes,
  networkTxInBytes: attributes.resources.network_tx_bytes,
  uptime: attributes.resources.uptime,
});

export const rawDataToServerDatabase = (data: any): ServerDatabase => ({
  id: data.id,
  name: data.name,
  username: data.username,
  connectionString: `${data.host.address}:${data.host.port}`,
  allowConnectionsFrom: data.connections_from,
  password: data.relationships.password?.attributes?.password,
});

export const rawDataToServerTask = (data: any): Task => ({
  id: data.id,
  sequenceId: data.sequence_id,
  action: data.action,
  payload: data.payload,
  timeOffset: data.time_offset,
  isQueued: data.is_queued,
  continueOnFailure: data.continue_on_failure,
  createdAt: new Date(data.created_at),
  updatedAt: new Date(data.updated_at),
});

export const rawDataToServerSchedule = (data: any): Schedule => ({
  id: data.id,
  name: data.name,
  cron: {
    dayOfWeek: data.cron.day_of_week,
    month: data.cron.month,
    dayOfMonth: data.cron.day_of_month,
    hour: data.cron.hour,
    minute: data.cron.minute,
  },
  isActive: data.is_active,
  isProcessing: data.is_processing,
  onlyWhenOnline: data.only_when_online,
  lastRunAt: data.last_run_at ? new Date(data.last_run_at) : null,
  nextRunAt: data.next_run_at ? new Date(data.next_run_at) : null,
  createdAt: new Date(data.created_at),
  updatedAt: new Date(data.updated_at),

  tasks: (data.relationships?.tasks?.data || []).map((row: any) => rawDataToServerTask(row.attributes)),
});

export const toCamelCase = (str: string): string => {
  return str.replace(/_([a-z])/g, (_, letter) => letter.toUpperCase());
};

export const transformKeysToCamelCase = <T>(obj: any): T => {
  if (Array.isArray(obj)) {
    return obj.map(item => transformKeysToCamelCase(item)) as unknown as T;
  } else if (obj !== null && typeof obj === 'object') {
    const newObj: Record<string, any> = {};
    for (const key in obj) {
      if (Object.prototype.hasOwnProperty.call(obj, key)) {
        const newKey = toCamelCase(key);
        newObj[newKey] = transformKeysToCamelCase(obj[key]);
      }
    }
    return newObj as T;
  }
  return obj;
};
