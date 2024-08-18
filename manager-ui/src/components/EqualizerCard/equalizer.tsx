import React, { forwardRef, useCallback, useImperativeHandle, useMemo, useState } from 'react';
import { Line } from 'react-chartjs-2';
import {
  CategoryScale,
  Chart as ChartJS,
  ChartData,
  Filler,
  Legend,
  LinearScale,
  LineElement,
  PointElement,
  Title,
  Tooltip
} from 'chart.js';
import 'chartjs-plugin-dragdata';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  Filler
);

export interface EqualizerProps {
  bands: number;
  input: number[];
  onEqualizerChange: (output: number[]) => void;
  disabled?: boolean;
}

export interface EqualizerRef {
  onReset: () => void;
}

export const MIN_VALUE = -6;
export const MAX_VALUE = 6;

export const Equalizer = forwardRef<EqualizerRef, EqualizerProps>((props, ref) => {
  const { bands, input, onEqualizerChange, disabled } = props;

  useImperativeHandle(ref, () => ({
    onReset: onResetClick
  }));

  const labels = ['100', '200', '400', '800', '1.6k', '3.2k', '6.4k', '12.8k', '16k', '20k'];
  const [dataSet, setDataSet] = useState<number[]>(input.slice(0, bands));

  const data: ChartData<'line', number[], string> = {
    labels: labels.slice(0, bands).map((label) => `${label}Hz`),
    datasets: [
      {
        data: dataSet,
        borderColor: '9B9B9B',
        borderWidth: 1,
        spanGaps: false,
        fill: true,
        backgroundColor: 'rgba(53, 162, 235, 0.5)',
        tension: 0.3,
        pointRadius: !disabled ? 2.3 : 0,
        pointHoverRadius: 3,
        pointBackgroundColor: '#609ACF',
        pointBorderWidth: 0
      }
    ]
  };

  const onDragEnd = (e: unknown, _datasetIdx: number, idx: string | number, value: number) => {
    if (disabled) return;

    const newDataSet = [...dataSet];
    newDataSet[idx as number] = value;
    setDataSet(newDataSet.slice(0, bands));
    onEqualizerChange(newDataSet);
  };

  const onResetClick = () => {
    const newDataSet = Array(bands).fill(0);
    setDataSet(newDataSet);
    onEqualizerChange(newDataSet);
  };

  // @ts-ignore
  const onHover = useCallback(
    (e) => {
      if (disabled) return;

      const point = e?.chart.getElementsAtEventForMode(e, 'nearest', { intersect: true }, false);
      if (point.length) e.native.target.style.cursor = 'grab';
      else e.native.target.style.cursor = 'default';
    },
    [disabled]
  );

  const options = useMemo(
    () => ({
      dragData: !disabled,
      scales: {
        y: {
          beginAtZero: true,
          max: MIN_VALUE,
          min: MAX_VALUE,
          grid: {
            display: false
          }
        },
        x: {
          grid: {
            display: false
          }
        }
      },
      onHover: onHover,
      plugins: {
        ...(!disabled && {
          dragData: {
            round: 1,
            dragX: true,
            showTooltip: true,
            onDragEnd: onDragEnd
          }
        }),
        legend: {
          display: false
        },
        title: {
          display: true,
          text: !disabled ? 'Custom EQ' : 'Preset EQ'
        }
      }
    }),
    [disabled, onHover]
  );

  return (
    <div className={'mt-3'}>
      <Line data={data} options={options} />
    </div>
  );
});

Equalizer.displayName = 'Equalizer';
