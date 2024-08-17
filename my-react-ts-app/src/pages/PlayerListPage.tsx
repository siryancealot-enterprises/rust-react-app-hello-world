import { Link } from "react-router-dom";
import BestTeamEver from "../components/BestTeamEver";
import PlayerListComponent from "../components/PlayerList";

export const PLAYER_LIST_URL = `/player-list`;

function PlayerListPage () {

    return <div className="App">
        <header className="App-header">
            <div><BestTeamEver /></div>
            <br/>
            <div>List of players: <PlayerListComponent/></div>
            <Link to={`/`} >
                <button>Go to home</button>
            </Link>
        </header>
    </div>

}

export default PlayerListPage