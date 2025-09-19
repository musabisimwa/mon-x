import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { 
  Container, Typography, Grid, Paper, Box, IconButton, Tabs, Tab,
  Card, CardContent, List, ListItem, ListItemText, Chip
} from '@mui/material';
import { ArrowBack, Timeline, Storage, BugReport, Memory } from '@mui/icons-material';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';
import axios from 'axios';

const AppDetailPage = () => {
  const { appName } = useParams();
  const navigate = useNavigate();
  const [tabValue, setTabValue] = useState(0);
  const [appData, setAppData] = useState(null);
  const [metrics, setMetrics] = useState([]);
  const [logs, setLogs] = useState([]);
  const [traces, setTraces] = useState([]);

  useEffect(() => {
    fetchAppData();
    const interval = setInterval(fetchAppData, 5000);
    return () => clearInterval(interval);
  }, [appName]);

  const fetchAppData = async () => {
    try {
      const [agentsRes, logsRes] = await Promise.all([
        axios.get('/api/agents'),
        axios.get(`/api/logs?q=agent_name:${appName}&size=50`)
      ]);
      
      const agent = agentsRes.data.data.find(a => a.name === appName);
      setAppData(agent);
      setLogs(logsRes.data.data?.hits?.hits || []);
      
      setMetrics(generateMockMetrics());
      setTraces(generateMockTraces());
    } catch (error) {
      console.error('Error fetching app data:', error);
    }
  };

  const generateMockMetrics = () => {
    return Array.from({ length: 20 }, (_, i) => ({
      time: new Date(Date.now() - (19 - i) * 60000).toLocaleTimeString(),
      cpu: Math.random() * 100,
      memory: Math.random() * 100,
      network: Math.random() * 1000,
      errors: Math.random() * 10
    }));
  };

  const generateMockTraces = () => {
    const operations = ['GET /api/users', 'POST /api/orders', 'GET /health', 'PUT /api/config'];
    return Array.from({ length: 10 }, (_, i) => ({
      id: `trace-${i}`,
      operation: operations[i % operations.length],
      duration: Math.floor(Math.random() * 500) + 10,
      status: Math.random() > 0.1 ? 'success' : 'error',
      timestamp: new Date(Date.now() - i * 30000).toISOString()
    }));
  };

  const renderTelemetry = () => (
    <Grid container spacing={3}>
      <Grid item xs={12}>
        <Paper sx={{ p: 2 }}>
          <Typography variant="h6" gutterBottom>Distributed Traces</Typography>
          <List>
            {traces.map((trace) => (
              <ListItem key={trace.id} divider>
                <ListItemText
                  primary={
                    <Box display="flex" justifyContent="space-between" alignItems="center">
                      <Typography variant="body1">{trace.operation}</Typography>
                      <Box>
                        <Chip 
                          label={`${trace.duration}ms`} 
                          size="small" 
                          color={trace.duration > 200 ? 'warning' : 'default'}
                        />
                        <Chip 
                          label={trace.status} 
                          size="small" 
                          color={trace.status === 'success' ? 'success' : 'error'}
                          sx={{ ml: 1 }}
                        />
                      </Box>
                    </Box>
                  }
                  secondary={`Trace ID: ${trace.id} • ${new Date(trace.timestamp).toLocaleString()}`}
                />
              </ListItem>
            ))}
          </List>
        </Paper>
      </Grid>
    </Grid>
  );

  const renderResources = () => (
    <Grid container spacing={3}>
      <Grid item xs={12} md={6}>
        <Paper sx={{ p: 2 }}>
          <Typography variant="h6" gutterBottom>CPU Usage</Typography>
          <ResponsiveContainer width="100%" height={200}>
            <LineChart data={metrics}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="time" />
              <YAxis />
              <Tooltip />
              <Line type="monotone" dataKey="cpu" stroke="#8884d8" strokeWidth={2} />
            </LineChart>
          </ResponsiveContainer>
        </Paper>
      </Grid>
      <Grid item xs={12} md={6}>
        <Paper sx={{ p: 2 }}>
          <Typography variant="h6" gutterBottom>Memory Usage</Typography>
          <ResponsiveContainer width="100%" height={200}>
            <LineChart data={metrics}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="time" />
              <YAxis />
              <Tooltip />
              <Line type="monotone" dataKey="memory" stroke="#82ca9d" strokeWidth={2} />
            </LineChart>
          </ResponsiveContainer>
        </Paper>
      </Grid>
    </Grid>
  );

  const renderLogs = () => (
    <Paper sx={{ p: 2 }}>
      <Typography variant="h6" gutterBottom>Application Logs</Typography>
      <List sx={{ maxHeight: 400, overflow: 'auto' }}>
        {logs.map((log, index) => {
          const source = log._source || {};
          return (
            <ListItem key={index} divider>
              <ListItemText
                primary={
                  <Box display="flex" justifyContent="space-between" alignItems="center">
                    <Typography variant="body2">{source.message || 'No message'}</Typography>
                    <Chip 
                      label={source.level || 'INFO'} 
                      size="small" 
                      color={source.level === 'ERROR' ? 'error' : source.level === 'WARN' ? 'warning' : 'default'}
                    />
                  </Box>
                }
                secondary={`${new Date(source.timestamp || source['@timestamp']).toLocaleString()} • ${source.service || appName}`}
              />
            </ListItem>
          );
        })}
      </List>
    </Paper>
  );

  if (!appData) {
    return (
      <Container maxWidth="xl" sx={{ py: 4 }}>
        <Typography>Loading...</Typography>
      </Container>
    );
  }

  return (
    <Container maxWidth="xl" sx={{ py: 4 }}>
      <Box display="flex" alignItems="center" mb={3}>
        <IconButton onClick={() => navigate('/')} sx={{ mr: 2 }}>
          <ArrowBack />
        </IconButton>
        <Typography variant="h4">{appName}</Typography>
        <Chip 
          label="ACTIVE" 
          color="success" 
          sx={{ ml: 2 }}
        />
      </Box>

      <Grid container spacing={3} mb={3}>
        <Grid item xs={12} sm={6} md={3}>
          <Card>
            <CardContent>
              <Typography color="textSecondary" gutterBottom>Status</Typography>
              <Typography variant="h6">Healthy</Typography>
            </CardContent>
          </Card>
        </Grid>
        <Grid item xs={12} sm={6} md={3}>
          <Card>
            <CardContent>
              <Typography color="textSecondary" gutterBottom>Capabilities</Typography>
              <Typography variant="h6">{Object.values(appData.capabilities).filter(Boolean).length}</Typography>
            </CardContent>
          </Card>
        </Grid>
        <Grid item xs={12} sm={6} md={3}>
          <Card>
            <CardContent>
              <Typography color="textSecondary" gutterBottom>Uptime</Typography>
              <Typography variant="h6">2h 34m</Typography>
            </CardContent>
          </Card>
        </Grid>
      </Grid>

      <Paper sx={{ width: '100%' }}>
        <Tabs 
          value={tabValue} 
          onChange={(e, newValue) => setTabValue(newValue)}
          sx={{ borderBottom: 1, borderColor: 'divider' }}
        >
          <Tab icon={<Timeline />} label="Telemetry" />
          <Tab icon={<Memory />} label="Resources" />
          <Tab icon={<BugReport />} label="Logs" />
        </Tabs>
        
        <Box sx={{ p: 3 }}>
          {tabValue === 0 && renderTelemetry()}
          {tabValue === 1 && renderResources()}
          {tabValue === 2 && renderLogs()}
        </Box>
      </Paper>
    </Container>
  );
};

export default AppDetailPage;
