import React from 'react'
import { Link } from "react-router-dom";
import PlayerListComponent from "../components/PlayerList";
import BestTeamEver from "../components/BestTeamEver";

const PlayerListPage = () => {

    return <div className="App">
        <header className="App-header">
            <div><BestTeamEver /></div>
            <div>List of players: <PlayerListComponent/></div>
            <Link to={`/`} >
                <button>Go to home</button>
            </Link>
        </header>
    </div>

}

export default PlayerListPage