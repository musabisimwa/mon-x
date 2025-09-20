import React, { useState, useEffect } from 'react';
import { Box, Typography, Chip, CircularProgress, Alert, Divider, List, ListItem, ListItemIcon, ListItemText } from '@mui/material';
import { Psychology, TrendingUp, Warning, CheckCircle, Build } from '@mui/icons-material';
import axios from 'axios';

const AIInsightsPanel = ({ agentId }) => {
  const [insights, setInsights] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    fetchInsights();
    const interval = setInterval(fetchInsights, 30000); // Refresh every 30 seconds
    return () => clearInterval(interval);
  }, [agentId]);

  const fetchInsights = async () => {
    try {
      setLoading(true);
      const response = await axios.get(`/api/ai-insights?agent_id=${agentId}`);
      if (response.data.success) {
        setInsights(response.data.data);
        setError(null);
      }
    } catch (err) {
      setError('Failed to fetch AI insights');
      console.error('AI insights error:', err);
    } finally {
      setLoading(false);
    }
  };

  const getSeverityColor = (severity) => {
    switch (severity?.toUpperCase()) {
      case 'CRITICAL': return 'error';
      case 'HIGH': return 'warning';
      case 'LOW': return 'success';
      default: return 'default';
    }
  };

  const getSeverityIcon = (severity) => {
    switch (severity?.toUpperCase()) {
      case 'CRITICAL': return <Warning color="error" />;
      case 'HIGH': return <TrendingUp color="warning" />;
      case 'LOW': return <CheckCircle color="success" />;
      default: return <Psychology />;
    }
  };

  if (loading) {
    return (
      <Box display="flex" justifyContent="center" alignItems="center" height={200}>
        <CircularProgress />
        <Typography variant="body2" sx={{ ml: 2 }}>
          AI analyzing system...
        </Typography>
      </Box>
    );
  }

  if (error) {
    return (
      <Alert severity="error" sx={{ mt: 2 }}>
        {error}
      </Alert>
    );
  }

  if (!insights) {
    return (
      <Typography color="textSecondary" sx={{ textAlign: 'center', mt: 4 }}>
        No AI insights available
      </Typography>
    );
  }

  return (
    <Box sx={{ height: '100%', overflow: 'auto' }}>
      {/* Severity Header */}
      <Box display="flex" alignItems="center" gap={1} mb={2}>
        {getSeverityIcon(insights.severity)}
        <Chip 
          label={insights.severity || 'UNKNOWN'} 
          color={getSeverityColor(insights.severity)}
          size="small"
          sx={{ fontWeight: 'bold' }}
        />
        <Typography variant="caption" color="textSecondary">
          {(insights.confidence * 100).toFixed(0)}% confidence
        </Typography>
      </Box>

      {/* Analysis */}
      <Box mb={2}>
        <Typography variant="body2" sx={{ 
          lineHeight: 1.5,
          p: 1.5,
          borderRadius: 1,
          borderLeft: `4px solid ${getSeverityColor(insights.severity) === 'error' ? '#f44336' : getSeverityColor(insights.severity) === 'warning' ? '#ff9800' : '#4caf50'}`
        }}>
          {insights.analysis}
        </Typography>
      </Box>

      <Divider sx={{ my: 2 }} />

      {/* Root Cause */}
      {insights.root_cause && (
        <Box mb={2}>
          <Typography variant="subtitle2" gutterBottom sx={{ fontWeight: 'bold' }}>
            Root Cause
          </Typography>
          <Typography variant="body2" color="textSecondary">
            {insights.root_cause}
          </Typography>
        </Box>
      )}

      {/* Suggested Fixes */}
      {insights.suggested_fixes && insights.suggested_fixes.length > 0 && (
        <Box>
          <Typography variant="subtitle2" gutterBottom sx={{ fontWeight: 'bold' }}>
             Suggested Actions
          </Typography>
          <List dense sx={{ py: 0 }}>
            {insights.suggested_fixes.map((fix, index) => (
              <ListItem key={index} sx={{ px: 0, py: 0.5 }}>
                <ListItemIcon sx={{ minWidth: 32 }}>
                  <Build fontSize="small" color="primary" />
                </ListItemIcon>
                <ListItemText 
                  primary={fix}
                  primaryTypographyProps={{ 
                    variant: 'body2',
                    sx: { fontSize: '0.875rem' }
                  }}
                />
              </ListItem>
            ))}
          </List>
        </Box>
      )}

      {/* Refresh Indicator */}
      <Box sx={{ position: 'absolute', bottom: 8, right: 8 }}>
        <Typography variant="caption" color="textSecondary">
          Auto-refresh: 30s
        </Typography>
      </Box>
    </Box>
  );
};

export default AIInsightsPanel;
