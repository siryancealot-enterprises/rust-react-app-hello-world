import { useEffect, useState } from "react";
import { API_URLS, APIConstants } from "../constants.ts";

export interface Player {
  id: string | undefined;
  number: number | undefined;
  name: string | undefined;
  email: string | undefined;
  username: string | undefined;
}

interface PlayerState {
  player: Player | null;
  loading: boolean;
  error: string | null;
}

const usePlayer = (id: string): PlayerState => {
  const [state, setState] = useState<PlayerState>({
    player: null,
    loading: true,
    error: null,
  });

  useEffect(() => {
    const fetchPlayer = async () => {
      try {
        const response = await fetch(
          APIConstants.BACKEND_BASE_URL + API_URLS.PLAYER_API + '/' + id
        );
        const json_response = await response.json();
        if (response.ok) {
          setState({ player: json_response, loading: false, error: null });
        } else {
          setState({ player: null, loading: false, error: json_response });
        }
      } catch (error) {
        setState({ player: null, loading: false, error: "Error retrieving: " + error });
      }
    };

    fetchPlayer();
  }, [id]);

  return state;
};

interface PlayerProps {
  id: string;
}

function PlayerComponent(props: PlayerProps) {

  const { player, loading, error } = usePlayer(props.id);

  if (loading) {
    return <><div>Loading...</div></>
  }

  if (error) {
    return <><div>Error retrieving: {error}</div></>
  }

  if (player) {
    return <>
      <table>
        <tr>
          <th>Name</th>
          <th>Number</th>
          <th>Email</th>
          <th>Username</th>
        </tr>
        <tr key={player.id}>
          <td>{player.name}</td>
          <td>{player.number}</td>
          <td>{player.email}</td>
          <td>{player.username}</td>
        </tr>
      </table>
    </>
  }

  return <></>
}

export default PlayerComponent;