import React from 'react';
import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Chip,
  Typography,
  Box
} from '@mui/material';

const LogsTable = ({ logs }) => {
  const getLevelColor = (level) => {
    switch (level?.toUpperCase()) {
      case 'ERROR': return 'error';
      case 'WARN': return 'warning';
      case 'INFO': return 'info';
      case 'DEBUG': return 'default';
      default: return 'default';
    }
  };

  if (!logs || logs.length === 0) {
    return (
      <Box sx={{ textAlign: 'center', py: 4 }}>
        <Typography variant="body2" color="textSecondary">
          No logs available
        </Typography>
      </Box>
    );
  }

  return (
    <TableContainer sx={{ maxHeight: 400 }}>
      <Table stickyHeader size="small">
        <TableHead>
          <TableRow>
            <TableCell>Timestamp</TableCell>
            <TableCell>Level</TableCell>
            <TableCell>Service</TableCell>
            <TableCell>Message</TableCell>
            <TableCell>Trace ID</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {logs.slice(0, 50).map((log, index) => {
            const source = log._source || {};
            return (
              <TableRow key={index} hover>
                <TableCell>
                  <Typography variant="caption">
                    {source.timestamp ? 
                      new Date(source.timestamp).toLocaleString() : 
                      new Date(source['@timestamp']).toLocaleString()
                    }
                  </Typography>
                </TableCell>
                <TableCell>
                  <Chip
                    label={source.level || 'INFO'}
                    size="small"
                    color={getLevelColor(source.level)}
                    variant="outlined"
                  />
                </TableCell>
                <TableCell>
                  <Typography variant="body2">
                    {source.service || 'unknown'}
                  </Typography>
                </TableCell>
                <TableCell>
                  <Typography 
                    variant="body2" 
                    sx={{ 
                      maxWidth: 300, 
                      overflow: 'hidden', 
                      textOverflow: 'ellipsis',
                      whiteSpace: 'nowrap'
                    }}
                  >
                    {source.message || 'No message'}
                  </Typography>
                </TableCell>
                <TableCell>
                  <Typography variant="caption" color="textSecondary">
                    {source.trace_id || '-'}
                  </Typography>
                </TableCell>
              </TableRow>
            );
          })}
        </TableBody>
      </Table>
    </TableContainer>
  );
};

export default LogsTable;
