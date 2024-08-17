import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import { API_URLS, APIConstants } from "../constants.ts";
import { getPlayerDetailURL } from "../pages/PlayerPage";
import { Player } from "./Player";

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
              APIConstants.BACKEND_BASE_URL + API_URLS.PLAYER_API
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
          <Link to={getPlayerDetailURL(player.id)} className='App-Link' >
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