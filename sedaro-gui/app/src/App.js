import React, { useEffect, useState } from 'react';
import Plot from 'react-plotly.js';

const App = () => {
  const [plotData, setPlotData] = useState([]); // Store plot data in state.
  useEffect(() => {
    // fetch plot data when the component mounts
    async function fetchData() {
      console.log('calling fetchdata...');
      try {
        // 'data.json' should be populated from a run of sim.py
        const response = await fetch('data.json');
        const data = await response.json();
        const updatedPlotData = {};
        // added agentId as name to make the legend meaningful TAC
        data.forEach(([t0, t1, frame]) => {
          for (let [agentId, { x, y }] of Object.entries(frame)) {
            updatedPlotData[agentId] = updatedPlotData[agentId] || { x: [], y: [], name: String };
            updatedPlotData[agentId].x.push(x);
            updatedPlotData[agentId].y.push(y);
            updatedPlotData[agentId].name = agentId;
          }
        });
        setPlotData(Object.values(updatedPlotData));
        console.log('plotData:', Object.values(updatedPlotData));
      } catch (error) {
        console.error('Error fetching data:', error);
      }
    }
    fetchData();
  }, []);

  return (
      <Plot
          style={{ position: 'fixed', width: '100%', height: '100%', left: 0, top: 0 }}
          data={plotData}
          layout={{
            title: { text: 'Sedaro Twin Demo - Simple GUI<br>(Two Body Problem?)' },
            // title: 'Sedaro Twin Demo\nTwo Body Problem?',
            yaxis: { scaleanchor: 'x' },
            autosize: true,
            showlegend: true,
            legend: {
              x: 1,
              y: 1,
              xanchor: 'right',
              yanchor: 'top',
              traceorder: 'normal',
              font: {
                family: 'sans-serif',
                size: 14,
                color: '#000',
              },
              bgcolor: '#f9f9f9',
              bordercolor: '#000000',
              borderwidth: 2,
            },
          }}
      />
  );
};
export default App;
