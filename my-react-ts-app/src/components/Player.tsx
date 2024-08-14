import { useState, useEffect } from "react";

export interface Player {
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

  const usePlayer = (number: String): PlayerState => {
    const [state, setState] = useState<PlayerState>({
      player: null,
      loading: true,
      error: null,
    });
  
    useEffect(() => {
      const fetchPlayer = async () => {
        try {
            const response = await fetch(
                "/api/players/" + number
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
    number: string;
  };

const Player: React.FC<PlayerProps> = ({ number }): JSX.Element  => {

    const { player, loading, error } = usePlayer(number);

    if (loading) {
      return <><div>Loading...</div></>
    }
  
    if (error) {
      return <><div>Error retrieving: {error}</div></> 
    }
  
    if (player) {
        return <>
        <div>
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