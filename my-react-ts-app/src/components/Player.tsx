import { useState, useEffect } from "react";

export interface Player {
    id: string | null;
    number: number;
    name: string;
    email: string;
    username: string;
}

interface PlayerState {
    player: Player | null;
    loading: boolean;
    error: string | null;
  }

  const usePlayer = (id: String): PlayerState => {
    const [state, setState] = useState<PlayerState>({
      player: null,
      loading: true,
      error: null,
    });
  
    useEffect(() => {
      const fetchPlayer = async () => {
        try {
            const response = await fetch(
                "/api/players/" + id
            );
            const json_response = await response.json();
            if (response.ok) {
                setState({ player: json_response, loading: false,error: null });
            } else {
                setState({ player: null, loading: false, error: json_response });
            }
        } catch (error) {
            setState({ player: null, loading: false, error: "Error retrieving: " + error });
        }
      };
  
      fetchPlayer();
    }, []);
  
    return state;
  };
  
  interface PlayerProps {
    id: string;
  };

const Player: React.FC<PlayerProps> = ({ id }): JSX.Element  => {

    const { player, loading, error } = usePlayer(id);

    if (loading) {
      return <><div>Loading...</div></>
    }
  
    if (error) {
      return <><div>Error retrieving: {error}</div></> 
    }
  
    if (player) {
        return <>
        <div>
            id: {player.id},
            Name: {player.name},
            Number: {player.number},
            Email: {player.email}, 
            Username: {player.username}
        </div>
        </>
    }

    return <></>
  }
  
export default Player;