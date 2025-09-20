import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import {
  Container, Grid, Paper, Typography, Box, IconButton, Card, CardContent,
  Chip, Alert, List, ListItem, ListItemText, Divider
} from '@mui/material';
import { ArrowBack, TrendingUp, Error, Speed, Memory } from '@mui/icons-material';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';
import AIInsightsPanel from './AIInsightsPanel';
import axios from 'axios';

const AppDetailPage = () => {
  const { appName } = useParams();
  const navigate = useNavigate();
  const [appData, setAppData] = useState(null);
  const [metrics, setMetrics] = useState([]);
  const [anomalies, setAnomalies] = useState([]);
  const [logs, setLogs] = useState([]);
  const [processes, setProcesses] = useState([]);

  useEffect(() => {
    fetchAppData();
    const interval = setInterval(fetchAppData, 5000);
    return () => clearInterval(interval);
  }, [appName]);

  const fetchAppData = async () => {
    try {
      const [agentsRes, anomaliesRes, logsRes] = await Promise.all([
        axios.get('/api/agents'),
        axios.get('/api/anomalies'),
        axios.get(`/api/agents/${appName}/logs?size=50`).catch(() => ({ data: { data: { hits: { hits: [] } } } }))
      ]);

      const agent = agentsRes.data.data.find(a => a.name === appName);
      setAppData(agent);

      // Filter anomalies for this specific app
      const appAnomalies = anomaliesRes.data.data.filter(a => 
        a.event?.source === appName || 
        a.event?.agent_id === appName ||
        a.agent_id === appName
      );
      setAnomalies(appAnomalies);

      // Get logs for this app
      const appLogs = logsRes.data.data?.hits?.hits || [];
      setLogs(appLogs);

      // Generate mock metrics for this specific app
      const now = Date.now();
      const mockMetrics = Array.from({ length: 20 }, (_, i) => ({
        time: new Date(now - (19 - i) * 30000).toLocaleTimeString(),
        cpu: Math.random() * 100,
        memory: Math.random() * 100,
        load: Math.random() * 4,
        errors: Math.floor(Math.random() * 10)
      }));
      setMetrics(mockMetrics);

    } catch (error) {
      console.error('Error fetching app data:', error);
    }
  };

  if (!appData) {
    return (
      <Container>
        <Typography>Loading application data...</Typography>
      </Container>
    );
  }

  const getHealthColor = () => {
    const lastSeen = new Date(appData.last_seen);
    const diffMinutes = (Date.now() - lastSeen) / (1000 * 60);
    if (diffMinutes < 2) return 'success';
    if (diffMinutes < 5) return 'warning';
    return 'error';
  };

  return (
    <Container maxWidth="xl" sx={{ py: 2 }}>
      {/* Header */}
      <Box display="flex" alignItems="center" mb={3}>
        <IconButton onClick={() => navigate('/')} sx={{ mr: 2 }}>
          <ArrowBack />
        </IconButton>
        <Box>
          <Typography variant="h4" component="h1">
            {appName}
          </Typography>
          <Typography variant="subtitle1" color="textSecondary">
            Application-specific monitoring dashboard
          </Typography>
        </Box>
        <Box sx={{ ml: 'auto' }}>
          <Chip 
            label={getHealthColor().toUpperCase()} 
            color={getHealthColor()} 
            sx={{ mr: 1 }} 
          />
          <Chip 
            label={`${anomalies.length} anomalies`}
            color={anomalies.length > 0 ? 'error' : 'success'}
            variant="outlined"
          />
        </Box>
      </Box>

      <Grid container spacing={3}>
        {/* Key Metrics Cards */}
        <Grid item xs={12} md={3}>
          <Card>
            <CardContent>
              <Box display="flex" alignItems="center">
                <Speed color="primary" sx={{ mr: 1 }} />
                <Typography variant="h6">CPU Load</Typography>
              </Box>
              <Typography variant="h3" color="primary">
                {metrics.length > 0 ? `${metrics[metrics.length - 1].cpu.toFixed(1)}%` : '0%'}
              </Typography>
              <Typography variant="caption" color="textSecondary">
                Application CPU usage
              </Typography>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} md={3}>
          <Card>
            <CardContent>
              <Box display="flex" alignItems="center">
                <Memory color="secondary" sx={{ mr: 1 }} />
                <Typography variant="h6">Memory</Typography>
              </Box>
              <Typography variant="h3" color="secondary">
                {metrics.length > 0 ? `${metrics[metrics.length - 1].memory.toFixed(1)}%` : '0%'}
              </Typography>
              <Typography variant="caption" color="textSecondary">
                Application memory usage
              </Typography>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} md={3}>
          <Card>
            <CardContent>
              <Box display="flex" alignItems="center">
                <Error color="error" sx={{ mr: 1 }} />
                <Typography variant="h6">Anomalies</Typography>
              </Box>
              <Typography variant="h3" color="error">
                {anomalies.length}
              </Typography>
              <Typography variant="caption" color="textSecondary">
                AI-detected issues
              </Typography>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} md={3}>
          <Card>
            <CardContent>
              <Box display="flex" alignItems="center">
                <TrendingUp color="success" sx={{ mr: 1 }} />
                <Typography variant="h6">Log Events</Typography>
              </Box>
              <Typography variant="h3" color="success">
                {logs.length}
              </Typography>
              <Typography variant="caption" color="textSecondary">
                Recent log entries
              </Typography>
            </CardContent>
          </Card>
        </Grid>

        {/* System Metrics Chart */}
        <Grid item xs={12} md={8}>
          <Paper sx={{ p: 2, height: 400 }}>
            <Typography variant="h6" gutterBottom>
              {appName} - System Metrics
            </Typography>
            <ResponsiveContainer width="100%" height="90%">
              <LineChart data={metrics}>
                <CartesianGrid strokeDasharray="3 3" />
                <XAxis dataKey="time" />
                <YAxis />
                <Tooltip />
                <Line type="monotone" dataKey="cpu" stroke="#1976d2" name="CPU %" />
                <Line type="monotone" dataKey="memory" stroke="#dc004e" name="Memory %" />
                <Line type="monotone" dataKey="load" stroke="#2e7d32" name="Load Avg" />
              </LineChart>
            </ResponsiveContainer>
          </Paper>
        </Grid>

        {/* AI Insights Panel */}
        <Grid item xs={12} md={4}>
          <Paper sx={{ p: 2, height: 400, position: 'relative' }}>
            <Typography variant="h6" gutterBottom>
              Intelligent Analysis
            </Typography>
            <AIInsightsPanel agentId={appName} />
        
            {anomalies.length === 0 ? (
              <Typography color="textSecondary">No anomalies detected</Typography>
            ) : (
              <List dense>
                {anomalies.map((anomaly, index) => (
                  <React.Fragment key={index}>
                    <ListItem>
                      <ListItemText
                        primary={
                          <Box>
                            <Chip 
                              label={anomaly.algorithm} 
                              size="small" 
                              color="warning" 
                              sx={{ mr: 1 }} 
                            />
                            <Typography variant="body2" component="span">
                              Score: {anomaly.score.toFixed(2)}
                            </Typography>
                          </Box>
                        }
                        secondary={
                          <Box>
                            <Typography variant="body2" color="textSecondary">
                              {anomaly.reason}
                            </Typography>
                            {anomaly.humanized && (
                              <Alert 
                                severity={anomaly.humanized.severity.toLowerCase() === 'critical' ? 'error' : 
                                         anomaly.humanized.severity.toLowerCase() === 'high' ? 'warning' : 'info'} 
                                sx={{ mt: 1, p: 1 }}
                              >
                                <Typography variant="caption">
                                  <strong>🔍 AI Analysis:</strong> {anomaly.humanized.human_explanation}
                                </Typography>
                                {anomaly.humanized.suggested_fixes.length > 0 && (
                                  <Typography variant="caption" display="block" sx={{ mt: 0.5 }}>
                                    <strong>💡 Fix:</strong> {anomaly.humanized.suggested_fixes[0]}
                                  </Typography>
                                )}
                                <Typography variant="caption" display="block" sx={{ mt: 0.5 }}>
                                  <strong>📊 Confidence:</strong> {(anomaly.humanized.confidence * 100).toFixed(0)}%
                                </Typography>
                              </Alert>
                            )}
                          </Box>
                        }
                      />
                    </ListItem>
                    {index < anomalies.length - 1 && <Divider />}
                  </React.Fragment>
                ))}
              </List>
            )}
          </Paper>
        </Grid>

        {/* Real-time Logs */}
        <Grid item xs={12}>
          <Paper sx={{ p: 2, height: 300, overflow: 'auto' }}>
            <Typography variant="h6" gutterBottom>
               {appName} - Logs ({logs.length})
            </Typography>
            <Box sx={{ fontFamily: 'monospace', fontSize: '0.875rem' }}>
              {logs.length === 0 ? (
                <Typography color="textSecondary">No recent logs for this application</Typography>
              ) : (
                logs.map((log, index) => {
                  const logData = log._source || log;
                  const levelColor = {
                    ERROR: '#f44336',
                    WARN: '#ff9800',
                    INFO: '#2196f3',
                    DEBUG: '#9e9e9e'
                  }[logData.level] || '#000';

                  return (
                    <Box key={index} sx={{ mb: 1, p: 1, bgcolor: 'grey.50', borderRadius: 1, borderLeft: `4px solid ${levelColor}` }}>
                      <Box display="flex" alignItems="center" gap={1}>
                        <Typography variant="caption" color="textSecondary" sx={{ minWidth: 80 }}>
                          {new Date(logData.timestamp * 1000).toLocaleTimeString()}
                        </Typography>
                        <Chip 
                          label={logData.level} 
                          size="small" 
                          sx={{ 
                            bgcolor: levelColor, 
                            color: 'white', 
                            minWidth: 60,
                            fontSize: '0.7rem'
                          }}
                        />
                        <Typography variant="body2" sx={{ flex: 1, wordBreak: 'break-word' }}>
                          {logData.message}
                        </Typography>
                      </Box>
                    </Box>
                  );
                })
              )}
            </Box>
          </Paper>
        </Grid>
      </Grid>
    </Container>
  );
};

export default AppDetailPage;
