import React from 'react'
import {Player} from '../components/PlayerList'; 

interface FormProps {
  onSubmit: (data: Player) => void;
}

export function AddPlayer({ onSubmit }: FormProps) {
  const [formData, setFormData] = React.useState<Player>({ name: '', number: 0, username: '', email: '' });

  function handleInputChange(event: React.ChangeEvent<HTMLInputElement>) {
    const { name, value } = event.target;
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
          const response = await fetch('/api/add_player', {
            method: 'POST',
            headers: {
              'Accept': 'application/json',
              'Content-Type': 'application/json',
            },
            body: JSON.stringify(formData)
          });
          const player = await response.json();
        //   setState({ players, loading: false, error: null });
        } catch (error) {
            console.log ('UI ERROR: ' + error);

            // TODO SWY: Do something with setState when we have it
        }
      };
  
      addPlayer();
  }

  return (
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
  );
}

export default AddPlayer;