import React from 'react'
import { Link } from "react-router-dom";
import {PlayerCreate} from '../components/PlayerCreate'; 
import { Player } from '../components/PlayerList';

function PlayerCreatePage() { 
  function handleSubmit(formData: Player) { 
    console.log(formData);
  }

  return (
    <div className="App">
        <header className="App-header">
            <div><PlayerCreate onSubmit={handleSubmit} /></div>
            <Link to={`/player-list`} >
                <button>Go to List of Players</button>
            </Link>
        </header>
    </div>
  );
  
}

export default PlayerCreatePage;