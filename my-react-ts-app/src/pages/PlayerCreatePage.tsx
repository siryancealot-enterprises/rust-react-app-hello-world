import React from 'react'
import { Link } from "react-router-dom";
import { Player } from '../components/Player';
import { getPlayerListURL } from './PlayerListPage';
import PlayerCreateComponent from '../components/PlayerCreate';

export function getPlayerCreateURL(): string {
  return `/player-create`;
}

function PlayerCreatePage() { 
  function handleSubmit(formData: Player) { 
    console.log(formData);
  }

  return (
    <div className="App">
        <header className="App-header">
            <div><PlayerCreateComponent onSubmit={handleSubmit} /></div>
            <Link to={getPlayerListURL()} >
                <button>Go to List of Players</button>
            </Link>
        </header>
    </div>
  );
  
}

export default PlayerCreatePage;