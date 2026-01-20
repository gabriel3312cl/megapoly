import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface GameState {
    user: {
        username: string;
        id: string;
    } | null;
    gameCode: string | null;
    setUser: (user: { username: string; id: string }) => void;
    setGameCode: (code: string | null) => void;
    logout: () => void;
}

export const useGameStore = create<GameState>()(
    persist(
        (set) => ({
            user: null,
            gameCode: null,
            setUser: (user) => set({ user }),
            setGameCode: (code) => set({ gameCode: code }),
            logout: () => set({ user: null, gameCode: null }),
        }),
        {
            name: 'megapoly-storage',
        }
    )
);
