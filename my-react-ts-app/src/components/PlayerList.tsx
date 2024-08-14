import { useState, useEffect } from "react";
import { Link } from "react-router-dom";
import { Player } from "./Player";
import { getPlayerPageURL } from "../pages/PlayerPage";

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
          setState({ players: null, loading: false, error: "Error retrieving list: " + error });
      }
    };

    fetchPlayers();
  }, []);

  return state;
};

function PlayerListComponent() {
  const { players, loading, error } = usePlayers();

  if (loading) {
    return <div>Loading...</div>;
  }

  if (error) {
    return <div>{error}</div>;
  }

  return (
    <ul>
      {players?.map((player)  => (
      <li key={player.id}>
          <Link to={getPlayerPageURL(player.id)} className='App-Link' >
              Name: {player.name}
          </Link>
          Number: {player.number},
          Email: {player.email}, 
          Username: {player.username}
      </li>
      ))}
    </ul>
  );
}

export default PlayerListComponent;