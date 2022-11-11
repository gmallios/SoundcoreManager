import React, { useEffect, useState } from "react";
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
} from 'chart.js';
import { Paper } from "@mui/material";
import useDeviceStore, { EQWave } from "../hooks/useDeviceStore";

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
);




export default function EQCard() {

  const options = {
    dragData: true,
    dragX: true,
    dragDataRound: 1,
    dragOptions: {
      showTooltip: false,
    },
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
    onHover(e) {
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
        onDragEnd: function (e, datasetIndex, index, value) {
          let newDataSet = dataSet;
          newDataSet[index] = value;
          setDataSet(newDataSet.slice(0, 8));
        }
      },
      legend: {
        display: false,
      },
    },

  };


  const labels = ["100", "200", "400", "800", "1.6k", "3.2k", "6.4k", "12.8kHz"];
  const [dataSet, setDataSet] = useState([0, 0, 0, 0, 0, 0, 0, 0]); /* Values are in dB -6 to 6 */
  const { deviceStatus, sendEQ } = useDeviceStore();

  function scale(number: number, inMin: number, inMax: number, outMin: number, outMax: number) {
    return (number - inMin) * (outMax - outMin) / (inMax - inMin) + outMin;
  }

  useEffect(() => {
    if (deviceStatus != undefined) {
      const newDataSet = [];
      Object.keys(deviceStatus.left_eq).forEach((key, _index) => {
        newDataSet.push(scale(deviceStatus.left_eq[key], 6, 18, -6, 6));
      });
      setDataSet(newDataSet.slice(0, 8));
    }
  }, []);

  useEffect(() => {
    console.log("DATA" + dataSet);
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
    sendEQ(eq);
  }, [dataSet]);

  const data = {
    labels: labels,
    datasets: [{
      data: dataSet,
      borderColor: '9B9B9B',
      borderWidth: 1,
      pointRadius: 5,
      pointHoverRadius: 5,
      pointBackgroundColor: '#609ACF',
      pointBorderWidth: 0,
      spanGaps: false,
    }],
  };

  return (
    <Paper sx={{ display: "flex", margin: 3, justifyContent: "center", alignItems: "center" }} >
      <Line data={data} options={options} />
    </Paper>
  );
}