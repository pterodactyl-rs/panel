const _CONVERSION_UNIT = 1024;

/**
 * Given a value in megabytes converts it back down into bytes.
 */
export function mbToBytes(megabytes: number): number {
  return Math.floor(megabytes * _CONVERSION_UNIT * _CONVERSION_UNIT);
}

/**
 * Given an amount of bytes, converts them into a human-readable string format
 * using "1024" as the divisor.
 */
export function bytesToString(bytes: number, decimals = 2): string {
  const k = _CONVERSION_UNIT;

  if (bytes < 1) return '0 Bytes';

  decimals = Math.floor(Math.max(0, decimals));
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  const value = Number((bytes / Math.pow(k, i)).toFixed(decimals));

  return `${value} ${['Bytes', 'KiB', 'MiB', 'GiB', 'TiB'][i]}`;
}
