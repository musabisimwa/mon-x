import React from 'react';
import { Grid, Card, CardContent, Typography, Chip, Box, LinearProgress, Paper } from '@mui/material';
import { CheckCircle, Error, Warning, Speed, Memory, BugReport, Visibility } from '@mui/icons-material';
import { useNavigate } from 'react-router-dom';

const AppDiscoveryWidget = ({ agents, anomalies = [] }) => {
  const navigate = useNavigate();

  const getHealthStatus = (agent) => {
    const lastSeen = new Date(agent.last_seen);
    const now = new Date();
    const diffMinutes = (now - lastSeen) / (1000 * 60);
    
    if (diffMinutes < 2) return { status: 'healthy', color: 'success', icon: CheckCircle };
    if (diffMinutes < 5) return { status: 'warning', color: 'warning', icon: Warning };
    return { status: 'error', color: 'error', icon: Error };
  };

  const getAppAnomalies = (agentName) => {
    return anomalies.filter(a => a.event?.source === agentName || a.event?.agent_id === agentName).length;
  };

  const handleViewApp = (appName) => {
    navigate(`/app/${appName}`);
  };

  const getMockMetrics = () => ({
    cpu: Math.random() * 100,
    memory: Math.random() * 100,
    load: Math.random() * 4
  });

  if (agents.length === 0) {
    return (
      <Paper sx={{ p: 6, textAlign: 'center', minHeight: '60vh', display: 'flex', flexDirection: 'column', justifyContent: 'center' }}>
        <Typography variant="h4" color="textSecondary" gutterBottom>
          Discovering Applications...
        </Typography>
        <Typography variant="body1" color="textSecondary">
          No applications detected yet. Start your monitoring agents to see them appear here.
        </Typography>
      </Paper>
    );
  }

  return (
    <Paper sx={{ p: 3, minHeight: '70vh' }}>
      <Box display="flex" alignItems="center" justifyContent="space-between" mb={4}>
        <Typography variant="h4" component="h2" sx={{ fontWeight: 'bold' }}>
          Discovered Applications
        </Typography>
        <Chip 
          label={`${agents.length} Active`} 
          color="primary" 
          size="large"
          sx={{ fontSize: '1.1rem', px: 2, py: 1 }}
        />
      </Box>

      <Grid container spacing={4}>
        {agents.map((agent) => {
          const health = getHealthStatus(agent);
          const HealthIcon = health.icon;
          const appAnomalies = getAppAnomalies(agent.name);
          const metrics = getMockMetrics();
          
          return (
            <Grid item xs={12} sm={6} md={4} lg={3} key={agent.name}>
              <Card 
                sx={{ 
                  height: '100%',
                  cursor: 'pointer',
                  transition: 'all 0.3s ease',
                  '&:hover': {
                    transform: 'translateY(-8px)',
                    boxShadow: 6,
                    '& .view-icon': {
                      opacity: 1
                    }
                  },
                  position: 'relative',
                  minHeight: 280
                }}
                onClick={() => handleViewApp(agent.name)}
              >
                <CardContent sx={{ p: 3, height: '100%', display: 'flex', flexDirection: 'column' }}>
                  {/* Header */}
                  <Box display="flex" justifyContent="space-between" alignItems="flex-start" mb={2}>
                    <Typography variant="h5" sx={{ fontWeight: 'bold', wordBreak: 'break-word', flex: 1 }}>
                      {agent.name}
                    </Typography>
                    <Visibility 
                      className="view-icon"
                      sx={{ 
                        opacity: 0, 
                        transition: 'opacity 0.3s',
                        color: 'primary.main',
                        ml: 1
                      }} 
                    />
                  </Box>
                  
                  {/* Health Status */}
                  <Box display="flex" alignItems="center" gap={1} mb={3}>
                    <HealthIcon color={health.color} />
                    <Chip 
                      label={health.status.toUpperCase()} 
                      color={health.color} 
                      size="small"
                      sx={{ fontWeight: 'bold' }}
                    />
                    {appAnomalies > 0 && (
                      <Chip 
                        icon={<BugReport />}
                        label={`${appAnomalies} issues`}
                        color="error"
                        size="small"
                        variant="outlined"
                      />
                    )}
                  </Box>

                  {/* Metrics */}
                  <Box mb={3} sx={{ flex: 1 }}>
                    <Box display="flex" alignItems="center" gap={1} mb={2}>
                      <Speed fontSize="small" color="primary" />
                      <Typography variant="body2" sx={{ minWidth: 40 }}>CPU:</Typography>
                      <LinearProgress 
                        variant="determinate" 
                        value={metrics.cpu} 
                        sx={{ flex: 1, height: 8, borderRadius: 4 }}
                        color={metrics.cpu > 80 ? 'error' : metrics.cpu > 60 ? 'warning' : 'primary'}
                      />
                      <Typography variant="body2" sx={{ minWidth: 40, textAlign: 'right' }}>
                        {metrics.cpu.toFixed(0)}%
                      </Typography>
                    </Box>
                    
                    <Box display="flex" alignItems="center" gap={1} mb={2}>
                      <Memory fontSize="small" color="secondary" />
                      <Typography variant="body2" sx={{ minWidth: 40 }}>RAM:</Typography>
                      <LinearProgress 
                        variant="determinate" 
                        value={metrics.memory} 
                        sx={{ flex: 1, height: 8, borderRadius: 4 }}
                        color={metrics.memory > 80 ? 'error' : metrics.memory > 60 ? 'warning' : 'secondary'}
                      />
                      <Typography variant="body2" sx={{ minWidth: 40, textAlign: 'right' }}>
                        {metrics.memory.toFixed(0)}%
                      </Typography>
                    </Box>
                  </Box>
                  
                  {/* Last Seen */}
                  <Typography variant="caption" color="textSecondary" sx={{ mb: 2 }}>
                    Last seen: {new Date(agent.last_seen).toLocaleTimeString()}
                  </Typography>
                  
                  {/* Capabilities */}
                  <Box>
                    {Object.entries(agent.capabilities).slice(0, 4).map(([cap, enabled]) => (
                      <Chip
                        key={cap}
                        label={cap}
                        size="small"
                        variant={enabled ? "filled" : "outlined"}
                        color={enabled ? "primary" : "default"}
                        sx={{ mr: 0.5, mb: 0.5, fontSize: '0.7rem' }}
                      />
                    ))}
                    {Object.keys(agent.capabilities).length > 4 && (
                      <Chip
                        label={`+${Object.keys(agent.capabilities).length - 4}`}
                        size="small"
                        variant="outlined"
                        sx={{ mr: 0.5, mb: 0.5, fontSize: '0.7rem' }}
                      />
                    )}
                  </Box>
                </CardContent>
              </Card>
            </Grid>
          );
        })}
      </Grid>
    </Paper>
  );
};

export default AppDiscoveryWidget;
