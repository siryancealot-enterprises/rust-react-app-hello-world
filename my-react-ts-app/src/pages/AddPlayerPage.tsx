import React from 'react'
import { Link } from "react-router-dom";
import {AddPlayer} from '../components/AddPlayer'; 
import { Player } from '../components/PlayerList';

function AddPlayerPage() {
  function handleSubmit(formData: Player) { 
    console.log(formData);
  }

  return (
    <div className="App">
        <header className="App-header">
            <div><AddPlayer onSubmit={handleSubmit} /></div>
            <Link to={`/players`} >
                <button>Go to List of Players</button>
            </Link>
        </header>
    </div>
  );
  
}

export default AddPlayerPage;