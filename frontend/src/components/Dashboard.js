import React, { useState, useEffect } from 'react';
import { Container, Grid, Paper, Typography } from '@mui/material';
import MetricsChart from './MetricsChart';
import AnomaliesPanel from './AnomaliesPanel';
import LogsTable from './LogsTable';
import AppDiscoveryWidget from './AppDiscoveryWidget';
import axios from 'axios';

function Dashboard() {
  const [metrics, setMetrics] = useState({});
  const [anomalies, setAnomalies] = useState([]);
  const [logs, setLogs] = useState([]);
  const [agents, setAgents] = useState([]);

  useEffect(() => {
    // Fetch initial data
    fetchMetrics();
    fetchAnomalies();
    fetchLogs();
    fetchAgents();

    // Setup WebSocket for real-time updates
    const ws = new WebSocket('ws://localhost:8080/ws');
    ws.onmessage = (event) => {
      const data = JSON.parse(event.data);
      if (data.type === 'anomalies') {
        setAnomalies(data.data);
      }
    };

    // Polling for metrics, logs, and agents
    const interval = setInterval(() => {
      fetchMetrics();
      fetchLogs();
      fetchAgents();
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

  const fetchAgents = async () => {
    try {
      const response = await axios.get('/api/agents');
      setAgents(response.data.data || []);
    } catch (error) {
      console.error('Error fetching agents:', error);
    }
  };

  return (
    <Container maxWidth="xl" sx={{ py: 4 }}>
      <Typography variant="h3" gutterBottom>
        Mon-X Dashboard
      </Typography>
      
      <Grid container spacing={3}>
        {/* App Discovery Widget */}
        <Grid item xs={12}>
          <Paper sx={{ p: 2 }}>
            <AppDiscoveryWidget agents={agents} />
          </Paper>
        </Grid>
        
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
  );
}

export default Dashboard;
