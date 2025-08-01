import {
  Chart as ChartJS,
  ChartData,
  ChartDataset,
  ChartOptions,
  Filler,
  LinearScale,
  LineElement,
  PointElement,
} from 'chart.js';
import { DeepPartial } from 'ts-essentials';
import { useState } from 'react';
import { deepmerge, deepmergeCustom } from 'deepmerge-ts';
import { hexToRgba } from '@/lib/color';

ChartJS.register(LineElement, PointElement, Filler, LinearScale);

const options: ChartOptions<'line'> = {
  responsive: true,
  animation: false,
  plugins: {
    legend: { display: false },
    title: { display: false },
    tooltip: { enabled: false },
  },
  layout: {
    padding: 0,
  },
  scales: {
    x: {
      min: 0,
      max: 19,
      type: 'linear',
      grid: {
        display: false,
      },
      ticks: {
        display: false,
      },
    },
    y: {
      min: 0,
      type: 'linear',
      grid: {
        display: true,
        color: '#4b5563', // gray-600
      },
      ticks: {
        display: true,
        count: 3,
        color: '#f3f4f6', // gray-100
        font: {
          size: 11,
          weight: 'lighter',
        },
      },
    },
  },
  elements: {
    point: {
      radius: 0,
    },
    line: {
      tension: 0.15,
    },
  },
};

function getOptions(opts?: DeepPartial<ChartOptions<'line'>> | undefined): ChartOptions<'line'> {
  return deepmerge(options, opts ?? {});
}

type ChartDatasetCallback = (value: ChartDataset<'line'>, index: number) => ChartDataset<'line'>;

function getEmptyData(label: string, sets = 1, callback?: ChartDatasetCallback | undefined): ChartData<'line'> {
  const next = callback || ((value) => value);

  return {
    labels: Array(20)
      .fill(0)
      .map((_, index) => index),
    datasets: Array(sets)
      .fill(0)
      .map((_, index) =>
        next(
          {
            fill: true,
            label,
            data: Array(20).fill(-5),
            borderColor: '#22d3ee', // cyan-400
            backgroundColor: hexToRgba('#0e7490', 0.5), // cyan-700
          },
          index,
        ),
      ),
  };
}

const merge = deepmergeCustom({ mergeArrays: false });

interface UseChartOptions {
  sets: number;
  options?: DeepPartial<ChartOptions<'line'>> | number | undefined;
  callback?: ChartDatasetCallback | undefined;
}

function useChart(label: string, opts?: UseChartOptions) {
  const options = getOptions(
    typeof opts?.options === 'number' ? { scales: { y: { min: 0, suggestedMax: opts.options } } } : opts?.options,
  );
  const [data, setData] = useState(getEmptyData(label, opts?.sets || 1, opts?.callback));

  const push = (items: number | null | (number | null)[]) =>
    setData((state) =>
      merge(state, {
        datasets: (Array.isArray(items) ? items : [items]).map((item, index) => ({
          ...state.datasets[index],
          data:
            state.datasets[index]?.data?.slice(1)?.concat(typeof item === 'number' ? Number(item.toFixed(2)) : item) ??
            [],
        })),
      }),
    );

  const clear = () =>
    setData((state) =>
      merge(state, {
        datasets: state.datasets.map((value) => ({
          ...value,
          data: Array(20).fill(-5),
        })),
      }),
    );

  return { props: { data, options }, push, clear };
}

function useChartTickLabel(label: string, max: number, tickLabel: string, roundTo?: number) {
  return useChart(label, {
    sets: 1,
    options: {
      scales: {
        y: {
          suggestedMax: max,
          ticks: {
            callback(value) {
              return `${roundTo ? Number(value).toFixed(roundTo) : value}${tickLabel}`;
            },
          },
        },
      },
    },
  });
}

export { useChart, useChartTickLabel, getOptions, getEmptyData };
