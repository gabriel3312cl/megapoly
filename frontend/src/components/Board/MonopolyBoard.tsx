'use client';

import React, { useEffect, useState } from 'react';
import Box from '@mui/material/Box';
import Paper from '@mui/material/Paper';
import Typography from '@mui/material/Typography';
import { boardData } from '../../data/boardData';

export default function MonopolyBoard() {
    const [backendStatus, setBackendStatus] = useState<string>('Connecting...');

    useEffect(() => {
        // Basic health check to backend
        fetch('http://localhost:3000/')
            .then(res => res.text())
            .then(data => setBackendStatus(data))
            .catch(err => setBackendStatus('Backend Offline'));
    }, []);

    // Mapping logic for 11x11 grid
    const getPosition = (index: number) => {
        if (index >= 0 && index <= 10) {
            return { gridRow: 11, gridColumn: 11 - index };
        }
        if (index >= 11 && index <= 20) {
            return { gridRow: 11 - (index - 10), gridColumn: 1 };
        }
        if (index >= 21 && index <= 30) {
            return { gridRow: 1, gridColumn: 1 + (index - 20) };
        }
        if (index >= 31 && index <= 39) {
            return { gridRow: 1 + (index - 30), gridColumn: 11 };
        }
        return { gridRow: 1, gridColumn: 1 };
    };

    return (
        <Box sx={{ flexGrow: 1, p: 2, height: '100vh', display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', bgcolor: '#222' }}>
            <Typography variant="h4" gutterBottom sx={{ color: 'white', fontFamily: 'monospace', textTransform: 'uppercase', letterSpacing: 4 }}>
                Megapoly
            </Typography>

            <Box sx={{
                display: 'grid',
                gridTemplateColumns: 'repeat(11, 1fr)',
                gridTemplateRows: 'repeat(11, 1fr)',
                gap: 0.5,
                width: '800px',
                height: '800px',
                bgcolor: '#cde6d0', // Slightly lighter board color
                border: '12px solid #333',
                p: 1,
                borderRadius: 2,
                boxShadow: '0 0 50px rgba(0,0,0,0.5)'
            }}>
                {/* Center Logo Area */}
                <Box sx={{ gridColumn: '2 / 11', gridRow: '2 / 11', display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', position: 'relative' }}>
                    <Typography variant="h1" sx={{
                        opacity: 0.1,
                        transform: 'rotate(-45deg)',
                        fontWeight: '900',
                        fontSize: '8rem',
                        userSelect: 'none'
                    }}>
                        MEGAPOLY
                    </Typography>

                    {/* Dice Area Placeholder + Backend Status */}
                    <Box sx={{ mt: 4, transform: 'rotate(-45deg)', p: 2, border: '2px dashed #888', borderRadius: 4, textAlign: 'center' }}>
                        <Typography variant="caption" display="block">Dice Area</Typography>
                        <Typography variant="caption" sx={{ color: backendStatus.includes('Online') ? 'green' : 'red', fontWeight: 'bold' }}>
                            [{backendStatus}]
                        </Typography>
                    </Box>
                </Box>

                {boardData.map((space) => {
                    const pos = getPosition(space.id);
                    const isCorner = space.type === 'Corner';

                    return (
                        <Paper key={space.id} sx={{
                            gridRow: pos.gridRow,
                            gridColumn: pos.gridColumn,
                            display: 'flex',
                            flexDirection: 'column',
                            justifyContent: 'space-between',
                            border: '1px solid #111',
                            bgcolor: '#fafafa',
                            overflow: 'hidden',
                            position: 'relative',
                            boxShadow: 'none',
                            borderRadius: 0,
                            // Corner styling
                            ...(isCorner && {
                                bgcolor: '#f0f0f0',
                                alignItems: 'center',
                                justifyContent: 'center'
                            })
                        }}>
                            {/* Color Bar */}
                            {space.type === 'Property' && space.color && (
                                <Box sx={{ height: '22%', bgcolor: space.color, borderBottom: '1px solid #111' }} />
                            )}

                            {/* Name */}
                            <Box sx={{
                                p: 0.5,
                                textAlign: 'center',
                                flexGrow: 1,
                                display: 'flex',
                                flexDirection: 'column',
                                justifyContent: isCorner ? 'center' : 'flex-start',
                                alignItems: 'center'
                            }}>
                                <Typography variant={isCorner ? "body1" : "caption"} sx={{
                                    lineHeight: 1.1,
                                    fontWeight: 'bold',
                                    fontSize: isCorner ? '0.8rem' : '0.65rem',
                                    mt: isCorner ? 0 : 0.5
                                }}>
                                    {space.name}
                                </Typography>
                                {/* Icon placeholder or detail */}
                                {space.type === 'Railroad' && (
                                    <Typography variant="caption" sx={{ fontSize: '1rem' }}>🚂</Typography>
                                )}
                                {space.type === 'Utility' && space.name.includes('Electric') && (
                                    <Typography variant="caption" sx={{ fontSize: '1rem' }}>💡</Typography>
                                )}
                                {space.type === 'Utility' && space.name.includes('Water') && (
                                    <Typography variant="caption" sx={{ fontSize: '1rem' }}>💧</Typography>
                                )}
                                {space.type === 'Action' && space.name.includes('Chance') && (
                                    <Typography variant="caption" sx={{ fontSize: '1rem' }}>❓</Typography>
                                )}
                                {space.type === 'Action' && space.name.includes('Community') && (
                                    <Typography variant="caption" sx={{ fontSize: '1rem' }}>📦</Typography>
                                )}
                            </Box>

                            {/* Price */}
                            {!isCorner && space.price && (
                                <Box sx={{ textAlign: 'center', p: 0.2, bgcolor: 'transparent' }}>
                                    <Typography variant="caption" sx={{ fontSize: '0.6rem' }}>${space.price}</Typography>
                                </Box>
                            )}
                        </Paper>
                    );
                })}
            </Box>
        </Box>
    );
}
