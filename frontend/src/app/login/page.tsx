'use client';
import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { useGameStore } from '@/store/useGameStore';
import {
    Box,
    Button,
    TextField,
    Typography,
    Paper,
    Tabs,
    Tab
} from '@mui/material';

export default function LoginPage() {
    const [tab, setTab] = useState(0);
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
    const [error, setError] = useState('');
    const router = useRouter();
    const setUser = useGameStore((state) => state.setUser);

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        setError('');

        const endpoint = tab === 0 ? '/login' : '/register';
        // Backend is on 8080
        const url = `http://localhost:8080${endpoint}`;

        try {
            const res = await fetch(url, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ username, password })
            });

            if (!res.ok) {
                const txt = await res.text();
                throw new Error(txt || 'Authentication failed');
            }

            const data = await res.json();
            setUser({ username: data.token, id: data.user_id });
            router.push('/lobby');
        } catch (err: any) {
            setError(err.message);
        }
    };

    return (
        <Box sx={{ height: '100vh', display: 'flex', alignItems: 'center', justifyContent: 'center', bgcolor: '#121212' }}>
            <Paper elevation={10} sx={{ p: 4, width: '400px', bgcolor: '#1e1e1e', color: 'white' }}>
                <Typography variant="h4" align="center" gutterBottom sx={{ fontFamily: 'monospace', fontWeight: 'bold' }}>
                    MEGAPOLY ID
                </Typography>

                <Tabs value={tab} onChange={(_, v) => setTab(v)} variant="fullWidth" sx={{ mb: 3 }} textColor="inherit" indicatorColor="primary">
                    <Tab label="Login" />
                    <Tab label="Register" />
                </Tabs>

                <form onSubmit={handleSubmit}>
                    <TextField
                        label="Username"
                        fullWidth
                        variant="outlined"
                        margin="normal"
                        value={username}
                        onChange={e => setUsername(e.target.value)}
                        InputProps={{ style: { color: 'white' } }}
                        InputLabelProps={{ style: { color: '#aaa' } }}
                        sx={{ fieldset: { borderColor: '#555' } }}
                    />
                    <TextField
                        label="Password"
                        type="password"
                        fullWidth
                        variant="outlined"
                        margin="normal"
                        value={password}
                        onChange={e => setPassword(e.target.value)}
                        InputProps={{ style: { color: 'white' } }}
                        InputLabelProps={{ style: { color: '#aaa' } }}
                        sx={{ fieldset: { borderColor: '#555' } }}
                    />

                    {error && <Typography color="error" variant="body2" sx={{ mt: 1 }}>{error}</Typography>}

                    <Button type="submit" variant="contained" fullWidth size="large" sx={{ mt: 3, fontWeight: 'bold' }}>
                        {tab === 0 ? 'Enter World' : 'Create Account'}
                    </Button>
                </form>
            </Paper>
        </Box>
    );
}
