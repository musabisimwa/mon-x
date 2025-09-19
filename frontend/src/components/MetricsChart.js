import React from 'react';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, BarChart, Bar } from 'recharts';
import { Grid, Card, CardContent, Typography } from '@mui/material';

const MetricsChart = ({ metrics }) => {
  // Mock time series data
  const timeSeriesData = Array.from({ length: 24 }, (_, i) => ({
    time: `${i}:00`,
    logs: Math.floor(Math.random() * 1000) + 500,
    errors: Math.floor(Math.random() * 50) + 10,
    responseTime: Math.floor(Math.random() * 100) + 50,
  }));

  return (
    <Grid container spacing={2} sx={{ height: '100%' }}>
      <Grid item xs={12} sm={6} md={3}>
        <Card>
          <CardContent>
            <Typography color="textSecondary" gutterBottom>
              Total Logs
            </Typography>
            <Typography variant="h4">
              {metrics.total_logs?.toLocaleString() || '0'}
            </Typography>
          </CardContent>
        </Card>
      </Grid>
      
      <Grid item xs={12} sm={6} md={3}>
        <Card>
          <CardContent>
            <Typography color="textSecondary" gutterBottom>
              Error Rate
            </Typography>
            <Typography variant="h4" color="error">
              {((metrics.error_rate || 0) * 100).toFixed(1)}%
            </Typography>
          </CardContent>
        </Card>
      </Grid>
      
      <Grid item xs={12} sm={6} md={3}>
        <Card>
          <CardContent>
            <Typography color="textSecondary" gutterBottom>
              Avg Response
            </Typography>
            <Typography variant="h4">
              {metrics.avg_response_time?.toFixed(1) || '0'}ms
            </Typography>
          </CardContent>
        </Card>
      </Grid>
      
      <Grid item xs={12} sm={6} md={3}>
        <Card>
          <CardContent>
            <Typography color="textSecondary" gutterBottom>
              Anomalies
            </Typography>
            <Typography variant="h4" color="warning.main">
              {metrics.anomalies_detected || 0}
            </Typography>
          </CardContent>
        </Card>
      </Grid>
      
      <Grid item xs={12}>
        <ResponsiveContainer width="100%" height={200}>
          <LineChart data={timeSeriesData}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="time" />
            <YAxis />
            <Tooltip />
            <Line type="monotone" dataKey="logs" stroke="#8884d8" strokeWidth={2} />
            <Line type="monotone" dataKey="errors" stroke="#ff7300" strokeWidth={2} />
          </LineChart>
        </ResponsiveContainer>
      </Grid>
    </Grid>
  );
};

export default MetricsChart;
