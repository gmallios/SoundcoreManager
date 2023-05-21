import React, { useEffect, useRef, useState } from "react";
import { Line } from "react-chartjs-2";
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
  Filler,
} from 'chart.js';
import { Button, Paper } from "@mui/material";
import useDeviceStore, { EQWave } from "../hooks/useDeviceStore";
import { useStatus, useUpdateEQ } from "../hooks/useSoundcoreDevice";

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  Filler,
);




export default function EQCard() {

  const options = {
    dragData: true,
    scales: {
      y: {
        beginAtZero: true,
        max: 6,
        min: -6,
        grid: {
          display: false,
        }
      },
      x: {
        grid: {
          display: false,
        }
      }
    },
    // Set cursor
    onHover(e: any) {
      const point = e.chart.getElementsAtEventForMode(
        e,
        'nearest',
        { intersect: true },
        false
      );
      if (point.length) e.native.target.style.cursor = 'grab';
      else e.native.target.style.cursor = 'default';
    },
    plugins: {
      dragData: {
        round: 1,
        dragX: true,
        showTooltip: true,
        onDragEnd: function (_e: any, _datasetIndex: any, index: string | number, value: number) {
          let newDataSet = dataSet;
          newDataSet[index as number] = value;
          shouldUpdate.current = true;
          setDataSet(newDataSet.slice(0, 8));
        }
      },
      legend: {
        display: false,
      },
      title: {
        display: true,
        text: 'EQ',
      },
    },

  };


  const labels = ["100", "200", "400", "800", "1.6k", "3.2k", "6.4k", "12.8kHz"];
  const [dataSet, setDataSet] = useState([0, 0, 0, 0, 0, 0, 0, 0]); /* Values are in dB -6 to 6 */
  const { data: status, isSuccess } = useStatus();
  const updateEQ = useUpdateEQ();
  const [isDataLoaded, setIsDataLoaded] = useState(false);
  const shouldUpdate = useRef(false);

  function scale(number: number, inMin: number, inMax: number, outMin: number, outMax: number) {
    return (number - inMin) * (outMax - outMin) / (inMax - inMin) + outMin;
  }

  function mapRange(value: number[], minInput: number, maxInput: number, minOutput: number, maxOutput: number) {
    const mappedArray = value.map((number) => {
      const normalizedValue = (number - minInput) / (maxInput - minInput);
      const mappedValue = normalizedValue * (maxOutput - minOutput) + minOutput;
      return mappedValue;
    });
    return mappedArray;
  }

  function resetEQ() {
    const eq: EQWave = {
      pos0: 12,
      pos1: 12,
      pos2: 12,
      pos3: 12,
      pos4: 12,
      pos5: 12,
      pos6: 12,
      pos7: 12,
      pos8: 12,
      pos9: 12,
    };
    updateEQ.mutate(eq);
    shouldUpdate.current = false;
    setDataSet([0, 0, 0, 0, 0, 0, 0, 0]);
  }

  useEffect(() => {
    if (status != undefined) {
      let newDataSet: number[] = [];
      const left_eq = Object.values(status.left_eq) as number[];
      console.log(left_eq);
      newDataSet = mapRange(left_eq, 6, 18, -6, 6);
      console.log(newDataSet);
      setDataSet(newDataSet.slice(0, 8));
      setIsDataLoaded(true);
    }
  }, [isSuccess]);

  useEffect(() => {
    if (shouldUpdate.current === true) {
      const eq: EQWave = {
        pos0: scale(dataSet[0], -6, 6, 6, 18),
        pos1: scale(dataSet[1], -6, 6, 6, 18),
        pos2: scale(dataSet[2], -6, 6, 6, 18),
        pos3: scale(dataSet[3], -6, 6, 6, 18),
        pos4: scale(dataSet[4], -6, 6, 6, 18),
        pos5: scale(dataSet[5], -6, 6, 6, 18),
        pos6: scale(dataSet[6], -6, 6, 6, 18),
        pos7: scale(dataSet[7], -6, 6, 6, 18),
        pos8: 12,
        pos9: 12,
      };
      updateEQ.mutate(eq);
    }
  }, [dataSet]);


  const data = {
    labels: labels,
    datasets: [{
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
      lineTension: 0.3,
    }],
  };

  return (
    <>
      <Paper sx={{ display: "flex", margin: 3, justifyContent: "center", alignItems: "center", marginBottom: 0 }}>
        {isDataLoaded && <><Line data={data} options={options} /></>}
      </Paper>
      <Button sx={{ margin: 0, padding: 0 }} onClick={resetEQ}>Reset</Button>
    </>
  );
}