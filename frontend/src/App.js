import React, { useState, useEffect } from 'react';
import { ThemeProvider, createTheme } from '@mui/material/styles';
import { CssBaseline, Container, Grid, Paper, Typography, Box } from '@mui/material';
import MetricsChart from './components/MetricsChart';
import AnomaliesPanel from './components/AnomaliesPanel';
import LogsTable from './components/LogsTable';
import axios from 'axios';

const theme = createTheme({
  palette: {
    mode: 'dark',
    primary: { main: '#1976d2' },
    secondary: { main: '#dc004e' },
  },
});

function App() {
  const [metrics, setMetrics] = useState({});
  const [anomalies, setAnomalies] = useState([]);
  const [logs, setLogs] = useState([]);

  useEffect(() => {
    // Fetch initial data
    fetchMetrics();
    fetchAnomalies();
    fetchLogs();

    // Setup WebSocket for real-time updates
    const ws = new WebSocket('ws://localhost:8080/ws');
    ws.onmessage = (event) => {
      const data = JSON.parse(event.data);
      if (data.type === 'anomalies') {
        setAnomalies(data.data);
      }
    };

    // Polling for metrics and logs
    const interval = setInterval(() => {
      fetchMetrics();
      fetchLogs();
    }, 10000);

    return () => {
      ws.close();
      clearInterval(interval);
    };
  }, []);

  const fetchMetrics = async () => {
    try {
      const response = await axios.get('/api/metrics');
      setMetrics(response.data.data);
    } catch (error) {
      console.error('Error fetching metrics:', error);
    }
  };

  const fetchAnomalies = async () => {
    try {
      const response = await axios.get('/api/anomalies');
      setAnomalies(response.data.data);
    } catch (error) {
      console.error('Error fetching anomalies:', error);
    }
  };

  const fetchLogs = async () => {
    try {
      const response = await axios.get('/api/logs?size=100');
      setLogs(response.data.data?.hits?.hits || []);
    } catch (error) {
      console.error('Error fetching logs:', error);
    }
  };

  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <Container maxWidth="xl" sx={{ py: 4 }}>
        <Typography variant="h3" gutterBottom>
          ML Monitoring Dashboard
        </Typography>
        
        <Grid container spacing={3}>
          <Grid item xs={12} md={8}>
            <Paper sx={{ p: 2, height: 400 }}>
              <Typography variant="h6" gutterBottom>
                System Metrics
              </Typography>
              <MetricsChart metrics={metrics} />
            </Paper>
          </Grid>
          
          <Grid item xs={12} md={4}>
            <Paper sx={{ p: 2, height: 400 }}>
              <Typography variant="h6" gutterBottom>
                Anomalies ({anomalies.length})
              </Typography>
              <AnomaliesPanel anomalies={anomalies} />
            </Paper>
          </Grid>
          
          <Grid item xs={12}>
            <Paper sx={{ p: 2 }}>
              <Typography variant="h6" gutterBottom>
                Recent Logs
              </Typography>
              <LogsTable logs={logs} />
            </Paper>
          </Grid>
        </Grid>
      </Container>
    </ThemeProvider>
  );
}

export default App;
