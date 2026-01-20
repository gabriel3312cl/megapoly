'use client';
import { useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { useGameStore } from '@/store/useGameStore';
import MonopolyBoard from "@/components/Board/MonopolyBoard";

export default function Home() {
  const router = useRouter();
  const gameCode = useGameStore((state) => state.gameCode);
  const user = useGameStore((state) => state.user);

  useEffect(() => {
    // If no user, login
    if (!user) {
      router.push('/login');
    } else if (!gameCode) {
      // If user but no game, lobby
      router.push('/lobby');
    }
  }, [user, gameCode, router]);

  if (!user || !gameCode) return null;

  return (
    <main>
      <MonopolyBoard />
    </main>
  );
}
