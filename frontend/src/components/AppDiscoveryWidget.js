import React from 'react';
import { Grid, Card, CardContent, Typography, Chip, Box, IconButton } from '@mui/material';
import { CheckCircle, Error, Warning, Visibility } from '@mui/icons-material';
import { useNavigate } from 'react-router-dom';

const AppDiscoveryWidget = ({ agents }) => {
  const navigate = useNavigate();

  const getHealthStatus = (agent) => {
    const lastSeen = new Date(agent.last_seen);
    const now = new Date();
    const diffMinutes = (now - lastSeen) / (1000 * 60);
    
    if (diffMinutes < 2) return { status: 'healthy', color: 'success', icon: CheckCircle };
    if (diffMinutes < 5) return { status: 'warning', color: 'warning', icon: Warning };
    return { status: 'error', color: 'error', icon: Error };
  };

  const handleViewApp = (appName) => {
    navigate(`/app/${appName}`);
  };

  return (
    <Box>
      <Typography variant="h6" gutterBottom>
        Discovered Applications ({agents.length})
      </Typography>
      <Grid container spacing={2}>
        {agents.map((agent) => {
          const health = getHealthStatus(agent);
          const HealthIcon = health.icon;
          
          return (
            <Grid item xs={12} sm={6} md={4} key={agent.name}>
              <Card sx={{ position: 'relative' }}>
                <CardContent>
                  <Box display="flex" justifyContent="space-between" alignItems="center">
                    <Typography variant="h6" noWrap>
                      {agent.name}
                    </Typography>
                    <IconButton 
                      size="small" 
                      onClick={() => handleViewApp(agent.name)}
                      color="primary"
                    >
                      <Visibility />
                    </IconButton>
                  </Box>
                  
                  <Box display="flex" alignItems="center" gap={1} mt={1}>
                    <HealthIcon color={health.color} fontSize="small" />
                    <Chip 
                      label={health.status.toUpperCase()} 
                      color={health.color} 
                      size="small" 
                    />
                  </Box>
                  
                  <Typography variant="caption" color="textSecondary" display="block" mt={1}>
                    Last seen: {new Date(agent.last_seen).toLocaleTimeString()}
                  </Typography>
                  
                  <Box mt={1}>
                    {Object.entries(agent.capabilities).map(([cap, enabled]) => (
                      <Chip
                        key={cap}
                        label={cap}
                        size="small"
                        variant={enabled ? "filled" : "outlined"}
                        color={enabled ? "primary" : "default"}
                        sx={{ mr: 0.5, mb: 0.5 }}
                      />
                    ))}
                  </Box>
                </CardContent>
              </Card>
            </Grid>
          );
        })}
      </Grid>
    </Box>
  );
};

export default AppDiscoveryWidget;
