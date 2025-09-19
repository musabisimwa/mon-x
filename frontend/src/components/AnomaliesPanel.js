import React from 'react';
import { List, ListItem, ListItemText, Chip, Box, Typography } from '@mui/material';
import { Warning, Error } from '@mui/icons-material';

const AnomaliesPanel = ({ anomalies }) => {
  if (!anomalies || anomalies.length === 0) {
    return (
      <Box sx={{ textAlign: 'center', py: 4 }}>
        <Typography variant="body2" color="textSecondary">
          No anomalies detected
        </Typography>
      </Box>
    );
  }

  return (
    <List sx={{ maxHeight: 320, overflow: 'auto' }}>
      {anomalies.slice(0, 10).map((anomaly, index) => (
        <ListItem key={index} divider>
          <Box sx={{ display: 'flex', alignItems: 'center', mr: 1 }}>
            {anomaly.score > 0.8 ? (
              <Error color="error" fontSize="small" />
            ) : (
              <Warning color="warning" fontSize="small" />
            )}
          </Box>
          <ListItemText
            primary={
              <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                <Typography variant="body2" noWrap>
                  {anomaly.event.service}
                </Typography>
                <Chip
                  label={`${(anomaly.score * 100).toFixed(0)}%`}
                  size="small"
                  color={anomaly.score > 0.8 ? 'error' : 'warning'}
                />
              </Box>
            }
            secondary={
              <Box>
                <Typography variant="caption" display="block">
                  {anomaly.reason}
                </Typography>
                <Typography variant="caption" color="textSecondary">
                  {new Date(anomaly.timestamp).toLocaleTimeString()}
                </Typography>
              </Box>
            }
          />
        </ListItem>
      ))}
    </List>
  );
};

export default AnomaliesPanel;
