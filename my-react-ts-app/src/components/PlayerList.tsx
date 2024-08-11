import { useState, useEffect } from "react";

interface Player {
  number: number;
  name: string;
  email: string;
  username: string;
}

interface PlayersState {
  players: Player[] | null;
  loading: boolean;
  error: string | null;
}

const usePlayers = (): PlayersState => {
  const [state, setState] = useState<PlayersState>({
    players: null,
    loading: true,
    error: null,
  });

  useEffect(() => {
    const fetchPlayers = async () => {
      try {
        const response = await fetch(
            "/api/players"
        );
        const players = await response.json();
        setState({ players, loading: false, error: null });
      } catch (error) {
        setState({ players: null, loading: false, error: "need to get the error message thing working below" });
        // setState({ players: null, loading: false, error: error.message });
      }
    };

    fetchPlayers();
  }, []);

  return state;
};

function PlayerList() {
  const { players, loading, error } = usePlayers();

  if (loading) {
    return <div>Loading...</div>;
  }

  if (error) {
    return <div>{error}</div>;
  }

  return (
    <ul>
      {players?.map((player) => (
      <li key={player.number}>
          Name: {player.name}, 
          Number: {player.number},
          Email: {player.email}, 
          Username: {player.username}
      </li>
      ))}
    </ul>
  );
}

export default PlayerList;