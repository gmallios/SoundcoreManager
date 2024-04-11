import { ChartData } from 'chart.js';
import { useCallback, useState } from 'react';
import { Line } from 'react-chartjs-2';
import 'chartjs-plugin-dragdata';
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  Filler
} from 'chart.js';

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
}

export const MIN_VALUE = -6;
export const MAX_VALUE = 6;

export const Equalizer = ({ bands, input, onEqualizerChange }: EqualizerProps): JSX.Element => {
  // The 9th and 10th bands are not verified to have that frequency
  const labels = ['100', '200', '400', '800', '1.6k', '3.2k', '6.4k', '12.8k', '16k', '20k'];
  const [dataSet, setDataSet] = useState<number[]>(input.slice(0, bands));

  const data: ChartData<'line', number[], string> = {
    labels: labels.slice(0, bands).map((label) => `${label}Hz`),
    datasets: [
      {
        data: dataSet,
        borderColor: '9B9B9B',
        borderWidth: 1,
        pointRadius: 2,
        pointHoverRadius: 3,
        pointBackgroundColor: '#609ACF',
        pointBorderWidth: 0,
        spanGaps: false,
        fill: true,
        backgroundColor: 'rgba(53, 162, 235, 0.5)',
        tension: 0.3
      }
    ]
  };

  const onDragEnd = (e: unknown, _datasetIdx: number, idx: string | number, value: number) => {
    const newDataSet = dataSet;
    newDataSet[idx as number] = value;
    setDataSet(newDataSet.slice(0, bands));
    onEqualizerChange(newDataSet);
    console.log('Equalizer output:', newDataSet);
  };

  //@ts-expect-error: no type found for this event
  const onHover = useCallback((e) => {
    const point = e?.chart.getElementsAtEventForMode(e, 'nearest', { intersect: true }, false);
    if (point.length) e.native.target.style.cursor = 'grab';
    else e.native.target.style.cursor = 'default';
  }, []);

  const options = {
    dragData: true,
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
        text: 'EQ'
      }
    }
  };

  return (
    <>
      <Line data={data} options={options} />
    </>
  );
};
