import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import { API_URLS, APIConstants } from '../constants.ts';
import { getPlayerDetailURL } from '../pages/PlayerPage';
import { Player } from './Player';

interface FormProps {
  onSubmit: (data: SearchTerm) => void;
}

interface PlayerResults {
  players: Player[] | null;
  error: string | null;
}

export interface SearchTerm {
  term: string | undefined;
}

export function PlayerSearchComponent({ onSubmit }: FormProps) {
  const [formData, setFormData] = React.useState<SearchTerm>({ term: undefined });

  const [state, setState] = useState<PlayerResults>({
    players: null,
    error: null,
  });

  const handleReset = () => {
    setState({ players: null, error: null });
  };

  function handleInputChange(event: React.ChangeEvent<HTMLInputElement>) {
    const { name } = event.target;
    setFormData({ ...formData, [name]: event.target.value });
  }

  function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    onSubmit(formData);

    // POST TO player search API
    const searchPlayer = async () => {
      try {
        const response = await fetch(APIConstants.BACKEND_BASE_URL + API_URLS.PLAYER_SEARCH_API, {
          method: APIConstants.POST,
          headers: {
            'Accept': APIConstants.APPLICATION_JSON_HEADER,
            'Content-Type': APIConstants.APPLICATION_JSON_HEADER,
          },
          body: JSON.stringify(formData)
        });
        const json_response = await response.json();
        if (response.ok) {
          setState({ players: json_response, error: null });
        } else {
          setState({ players: null, error: json_response });
        }
      } catch (error) {
        setState({ players: null, error: 'Player search error: ' + error });
      }
    };

    searchPlayer();
  }

  if (state.players) {
    if (state.players?.length == 0) {
      return (
        <div>
          <h2>Your search results</h2>
          <div>No results found!</div>
          <br />
          <div>
            <button onClick={handleReset}>Search again</button>
          </div>
        </div>
      );
    } else {
      return (
        <div>
          <h2>Your search results</h2>
          <table>
            <tr>
              <th>Name</th>
              <th>Number</th>
              <th>Email</th>
              <th>Username</th>
            </tr>
            {state.players?.map((player) => (
              <tr key={player.id}>
                <td><Link to={getPlayerDetailURL(player.id)} className='App-Link' >{player.name}</Link></td>
                <td>{player.number}</td>
                <td><Link to={'mailto:' + (player.email)}>{player.email}</Link></td>
                <td>{player.username}</td>
              </tr>
            ))}
          </table>
          <br />
          <div>
            <button onClick={handleReset}>Search again</button>
          </div>
        </div >
      );
    }
  }

  if (state.error) {
    return <div>Player search error: {state.error}</div>;
  }

  return (
    <div>
      <h2>Enter your player search term(s):</h2>
      <div>
        <form onSubmit={handleSubmit}>
          <label>
            <input type="text" name="term" value={formData.term} onChange={handleInputChange} />
          </label>
          <br />
          <button type="submit">Submit</button>
        </form>
      </div>
    </div >
  );
}

export default PlayerSearchComponent;