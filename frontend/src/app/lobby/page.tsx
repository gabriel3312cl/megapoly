'use client';
import { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import { useGameStore } from '@/store/useGameStore';
import {
    Box,
    Button,
    TextField,
    Typography,
    Paper,
    Grid,
    List,
    ListItem,
    ListItemText,
    Divider
} from '@mui/material';

export default function LobbyPage() {
    const user = useGameStore((state) => state.user);
    const setGameCode = useGameStore((state) => state.setGameCode);
    const router = useRouter();
    const [joinCode, setJoinCode] = useState('');
    const [publicGames, setPublicGames] = useState<string[]>([]);

    useEffect(() => {
        if (!user) {
            router.push('/login');
            return;
        }

        // Fetch games
        fetch('http://localhost:8080/games')
            .then(res => res.json())
            .then(data => setPublicGames(data))
            .catch(console.error);
    }, [user, router]);

    const handleCreateGame = async () => {
        if (!user) return;
        try {
            const res = await fetch('http://localhost:8080/game/create', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ host_id: user.id })
            });
            const data = await res.json();
            setGameCode(data.game_code);
            router.push('/');
        } catch (err) {
            console.error(err);
        }
    };

    const handleJoinGame = async () => {
        if (!user || joinCode.length !== 4) return;
        try {
            const res = await fetch('http://localhost:8080/game/join', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ user_id: user.id, game_code: joinCode.toUpperCase() })
            });
            if (res.ok) {
                setGameCode(joinCode.toUpperCase());
                router.push('/');
            } else {
                alert('Game not found');
            }
        } catch (err) {
            console.error(err);
        }
    };

    if (!user) return null;

    return (
        <Box sx={{ minHeight: '100vh', bgcolor: '#121212', color: 'white', p: 4 }}>
            <Box sx={{ maxWidth: '800px', mx: 'auto' }}>
                <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 6 }}>
                    <Typography variant="h3" sx={{ fontFamily: 'monospace', fontWeight: 'bold' }}>LOBBY</Typography>
                    <Box sx={{ textAlign: 'right' }}>
                        <Typography variant="h6">{user.username}</Typography>
                        <Button color="error" onClick={() => { useGameStore.getState().logout(); router.push('/login'); }}>Logout</Button>
                    </Box>
                </Box>

                <Grid container spacing={4}>
                    <Grid item xs={12} md={6}>
                        <Paper sx={{ p: 4, bgcolor: '#1e1e1e', height: '100%', color: 'white' }}>
                            <Typography variant="h5" gutterBottom>Start New Journey</Typography>
                            <Typography variant="body2" sx={{ mb: 4, color: '#aaa' }}>
                                Create a new game session and invite players with your unique 4-digit code.
                            </Typography>
                            <Button variant="contained" fullWidth size="large" onClick={handleCreateGame} sx={{ py: 2, fontSize: '1.2rem' }}>
                                Create Game
                            </Button>
                        </Paper>
                    </Grid>

                    <Grid item xs={12} md={6}>
                        <Paper sx={{ p: 4, bgcolor: '#1e1e1e', height: '100%', color: 'white' }}>
                            <Typography variant="h5" gutterBottom>Join Existing</Typography>
                            <Box sx={{ display: 'flex', gap: 2, mb: 4 }}>
                                <TextField
                                    label="GAME CODE"
                                    value={joinCode}
                                    onChange={e => setJoinCode(e.target.value.toUpperCase())}
                                    inputProps={{ maxLength: 4, style: { textAlign: 'center', fontSize: '1.5rem', letterSpacing: 5 } }}
                                    fullWidth
                                    sx={{ input: { color: 'white' }, label: { color: '#aaa' }, fieldset: { borderColor: '#555' } }}
                                />
                                <Button variant="contained" color="secondary" onClick={handleJoinGame} disabled={joinCode.length !== 4}>
                                    JOIN
                                </Button>
                            </Box>

                            <Divider sx={{ mb: 2, borderColor: '#333' }} />
                            <Typography variant="subtitle2" sx={{ mb: 2, color: '#888' }}>Active Public Games</Typography>

                            <List dense sx={{ maxHeight: '200px', overflow: 'auto' }}>
                                {publicGames.map(code => (
                                    <ListItem key={code} secondaryAction={
                                        <Button size="small" onClick={() => { setJoinCode(code); }}>Use</Button>
                                    }>
                                        <ListItemText primary={`Game #${code}`} secondary="Waiting for players..." secondaryTypographyProps={{ color: '#666' }} />
                                    </ListItem>
                                ))}
                                {publicGames.length === 0 && <Typography variant="caption" sx={{ color: '#555' }}>No active games found.</Typography>}
                            </List>
                        </Paper>
                    </Grid>
                </Grid>
            </Box>
        </Box>
    );
}
