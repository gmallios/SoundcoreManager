import React, { useState } from "react";
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


ChartJS.register(
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend
  );
  



export default function EQCard(props) {

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
                console.log(datasetIndex, index, value)
              }
          },
          legend: {
            display: false,
          },
        },
        
      };


    const labels = ["100", "200", "400", "800", "1.6k", "3.2k", "6.4k", "12.8kHz"];
    const [dataSet, setDataSet] = useState([0, 0, 0, 0, 0, 0, 0, 0]);



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