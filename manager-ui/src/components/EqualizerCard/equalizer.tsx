import { semanticColors } from '@nextui-org/react';
import {
  BubbleDataPoint,
  CategoryScale,
  Chart,
  ChartData,
  ChartEvent,
  Chart as ChartJS,
  ChartOptions,
  Filler,
  Legend,
  LinearScale,
  LineElement,
  Point,
  PointElement,
  Title,
  Tooltip
} from 'chart.js';
import { ActiveElement } from 'chart.js/dist/plugins/plugin.tooltip';
import 'chartjs-plugin-dragdata';
import { forwardRef, useImperativeHandle } from 'react';
import { Line } from 'react-chartjs-2';

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

  const data: ChartData<'line', number[], string> = {
    labels: labels.slice(0, bands).map((label) => `${label}Hz`),
    datasets: [
      {
        data: [...input],
        spanGaps: false,
        fill: true,
        backgroundColor: semanticColors.dark.primary[200],
        tension: 0.3,
        pointRadius: !disabled ? 3 : 0,
        pointHoverRadius: 3,
        pointBackgroundColor: semanticColors.dark.primary[500],
        pointBorderWidth: 0
      }
    ]
  };

  const onDragEnd = (
    _e: unknown,
    _datasetIdx: number,
    idx: number,
    value: number | Point | [number, number] | BubbleDataPoint | null
  ) => {
    if (disabled || value === null || typeof value != 'number') return;
    const newDataSet = [...input];
    newDataSet[idx as number] = value;
    onEqualizerChange(newDataSet);
  };

  const onResetClick = () => {
    const newDataSet = Array(bands).fill(0);
    onEqualizerChange(newDataSet);
  };

  const onHover = (e: ChartEvent, _elements: ActiveElement[], chart: Chart) => {
    if (disabled) return;

    const point = chart.getElementsAtEventForMode(
      e as unknown as Event,
      'nearest',
      { intersect: true },
      false
    );

    if (point.length) (e.native?.target as HTMLElement).style.cursor = 'grab';
    else (e.native?.target as HTMLElement).style.cursor = 'default';
  };

  const options: ChartOptions<'line'> = {
    scales: {
      y: {
        beginAtZero: true,
        suggestedMin: MIN_VALUE,
        suggestedMax: MAX_VALUE,
        grid: {
          display: true
        },
        ticks: {
          display: true,
          stepSize: 1,
          autoSkip: false
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
      dragData: {
        round: 1,
        dragX: true,
        showTooltip: true,
        onDragEnd: onDragEnd
      },
      legend: {
        display: false
      },
      title: {
        display: true,
        text: !disabled ? 'Custom EQ' : 'Preset EQ'
      }
    },
    responsive: true,
    maintainAspectRatio: false
  };

  return (
    <div className={'mt-3 min-h-[500px]'}>
      <Line data={data} options={options} />
    </div>
  );
});

Equalizer.displayName = 'Equalizer';
