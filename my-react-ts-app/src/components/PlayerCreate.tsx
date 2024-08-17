import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import { API_URLS, APIConstants } from '../constants.ts';
import { getPlayerDetailURL } from '../pages/PlayerPage';
import { Player } from './Player';

interface FormProps {
  onSubmit: (data: Player) => void;
}

interface CreatedPlayer {
  player: Player | null;
  error: string | null;
}

export function PlayerCreateComponent({ onSubmit }: FormProps) {
  const [formData, setFormData] = React.useState<Player>({ id: null, name: '', number: 0, username: '', email: '' });

  const [state, setState] = useState<CreatedPlayer>({
    player: null,
    error: null,
  });

  function handleInputChange(event: React.ChangeEvent<HTMLInputElement>) {
    const { name } = event.target;
    // TODO SWY: This line below is to handle "number" types in forms coming across with values as strings,
    // which JSON.stringify sends over in double quotes, which makes the server side JSON parser throw up. 
    // see: https://stackoverflow.com/questions/61070803/how-to-handle-number-input-in-typescript
    const processedValue = event.target.type === "number" && !isNaN(event.target.valueAsNumber) ? event.target.valueAsNumber : event.target.value
    setFormData({ ...formData, [name]: processedValue });
  }

  function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    onSubmit(formData);

    // POST TO add_player API
    const addPlayer = async () => {
      try {
        const response = await fetch(APIConstants.BACKEND_BASE_URL + API_URLS.PLAYER_API, {
          method: APIConstants.POST,
          headers: {
            'Accept': APIConstants.APPLICATION_JSON_HEADER,
            'Content-Type': APIConstants.APPLICATION_JSON_HEADER,
          },
          body: JSON.stringify(formData)
        });
        const json_response = await response.json();
        if (response.ok) {
          setState({ player: json_response, error: null });
        } else {
          setState({ player: null, error: json_response });
        }
      } catch (error) {
        setState({ player: null, error: 'Player creation error: ' + error });
      }
    };

    addPlayer();
  }

  if (state.player) {
    return <div>
      Created player: <Link to={getPlayerDetailURL(state.player.id)} className='App-Link' >
        {state.player.name}
      </Link>, with number: {state.player.number}
    </div>;
  }

  if (state.error) {
    return <div>Player creation error: {state.error}</div>;
  }

  return (
    <div>
      <h2>Add your player details:</h2>
      <form onSubmit={handleSubmit}>
        <label>
          Name:
          <input type="text" name="name" value={formData.name} onChange={handleInputChange} />
        </label>
        <br />
        <label>
          Number:
          <input type="number" name="number" value={formData.number} onChange={handleInputChange} />
        </label>
        <br />
        <label>
          Username:
          <input type="text" name="username" value={formData.username} onChange={handleInputChange} />
        </label>
        <br />
        <label>
          Email:
          <input type="email" name="email" value={formData.email} onChange={handleInputChange} />
        </label>
        <br />
        <button type="submit">Submit</button>
      </form>
    </div>
  );
}

export default PlayerCreateComponent;