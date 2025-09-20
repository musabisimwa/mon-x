import React, { useState, useEffect } from 'react';
import { Container, Typography, Box } from '@mui/material';
import AppDiscoveryWidget from './AppDiscoveryWidget';
import axios from 'axios';

function Dashboard() {
  const [agents, setAgents] = useState([]);
  const [anomalies, setAnomalies] = useState([]);

  useEffect(() => {
    fetchData();
    const interval = setInterval(fetchData, 5000);
    return () => clearInterval(interval);
  }, []);

  const fetchData = async () => {
    try {
      const [agentsRes, anomaliesRes] = await Promise.all([
        axios.get('/api/agents'),
        axios.get('/api/anomalies')
      ]);
      
      setAgents(agentsRes.data.data || []);
      setAnomalies(anomaliesRes.data.data || []);
    } catch (error) {
      console.error('Error fetching data:', error);
    }
  };

  return (
    <Container maxWidth="xl" sx={{ py: 3, height: '100vh' }}>
      <Box mb={4}>
        <Typography variant="h3" component="h1" gutterBottom sx={{ fontWeight: 'bold' }}>
          Mon-X Monitoring Dashboard
        </Typography>
        <Typography variant="h6" color="textSecondary">
          Real-time application monitoring with AI-powered insights
        </Typography>
      </Box>

      <AppDiscoveryWidget agents={agents} anomalies={anomalies} />
    </Container>
  );
}

export default Dashboard;
