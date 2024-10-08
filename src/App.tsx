// src/App.tsx

import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  PieChart,
  Pie,
  Cell,
  Tooltip,
  Legend,
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  LineChart,
  Line,
} from "recharts";
import "./App.css";

// Define the type for a usage record
interface UsageRecord {
  app: string;
  window: string;
  start_time: string;
  end_time: string;
  duration_secs: number;
}

function App(): JSX.Element {
  const [usageData, setUsageData] = useState<UsageRecord[]>([]);
  const [isTracking, setIsTracking] = useState<boolean>(false);

  useEffect(() => {
    let intervalId: ReturnType<typeof setInterval>;

    if (isTracking) {
      // Fetch data immediately and then every 5 seconds
      fetchUsageData();
      intervalId = setInterval(fetchUsageData, 5000);
    }

    return () => {
      if (intervalId) clearInterval(intervalId);
    };
  }, [isTracking]);

  const startTracking = async (): Promise<void> => {
    try {
      await invoke("start_tracking");
      setIsTracking(true);
    } catch (error) {
      console.error("Failed to start tracking:", error);
    }
  };

  const stopTracking = async (): Promise<void> => {
    try {
      await invoke("stop_tracking");
      setIsTracking(false);
      setUsageData([]); // Clear data when tracking stops
    } catch (error) {
      console.error("Failed to stop tracking:", error);
    }
  };

  const fetchUsageData = async (): Promise<void> => {
    try {
      const data = await invoke<UsageRecord[]>("get_usage_data");
      setUsageData(data);
    } catch (error) {
      console.error("Failed to fetch usage data:", error);
    }
  };

  // Prepare data for the charts
  const totalTimePerApp = usageData.reduce<Record<string, number>>(
    (acc, record) => {
      acc[record.app] = (acc[record.app] || 0) + record.duration_secs;
      return acc;
    },
    {}
  );

  const pieChartData = Object.entries(totalTimePerApp).map(
    ([app, totalDuration]) => ({
      app,
      totalDuration,
    })
  );

  const COLORS = [
    "#8884d8",
    "#82ca9d",
    "#ffc658",
    "#ff7f50",
    "#8dd1e1",
    "#a4de6c",
    "#d0ed57",
  ];

  // Line chart data
  const lineChartData = usageData.map((record) => ({
    time: new Date(record.end_time).toLocaleTimeString(),
    duration: record.duration_secs,
    app: record.app,
  }));

  return (
    <div className="container">
      <h1>Productivity App</h1>

      <div className="controls">
        {!isTracking ? (
          <button onClick={startTracking}>Start Tracking</button>
        ) : (
          <button onClick={stopTracking}>Stop Tracking</button>
        )}
      </div>

      <h2>Usage Data</h2>

      {usageData.length > 0 ? (
        <>
          {/* Pie Chart */}
          <h3>Total Time Spent per Application</h3>
          <PieChart width={400} height={400}>
            <Pie
              data={pieChartData}
              dataKey="totalDuration"
              nameKey="app"
              cx="50%"
              cy="50%"
              outerRadius={150}
              fill="#8884d8"
              label
            >
              {pieChartData.map((entry, index) => (
                <Cell
                  key={`cell-${index}`}
                  fill={COLORS[index % COLORS.length]}
                />
              ))}
            </Pie>
            <Tooltip />
            <Legend />
          </PieChart>

          {/* Bar Chart */}
          <h3>Duration of Each Session</h3>
          <BarChart width={600} height={300} data={usageData}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="window" />
            <YAxis />
            <Tooltip />
            <Legend />
            <Bar dataKey="duration_secs" name="Duration (s)" fill="#82ca9d" />
          </BarChart>

          {/* Line Chart */}
          <h3>Application Usage Over Time</h3>
          <LineChart width={600} height={300} data={lineChartData}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="time" />
            <YAxis />
            <Tooltip />
            <Legend />
            <Line
              type="monotone"
              dataKey="duration"
              name="Duration (s)"
              stroke="#8884d8"
            />
          </LineChart>
        </>
      ) : isTracking ? (
        <p>Loading usage data...</p>
      ) : (
        <p>Tracking is not active.</p>
      )}
    </div>
  );
}

export default App;
