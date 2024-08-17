import { Link, useParams } from "react-router-dom";
import PlayerComponent from "../components/Player";
import { PLAYER_LIST_URL } from './PlayerListPage';

export function getPlayerDetailURL(id : string | null): string {
    return `/player/` + id;
}

export const PLAYER_DETAIL_URL_FORMAT =  `/player/:id`;


function PlayerPage () { 
    const props = useParams();
    if  (props.id === undefined) {
        return <><div>Missing player "id" in the URL!</div></> 
    } else {
        return <div className="App">
            <header className="App-header">
                <h2>Here's your player:</h2>
                <br/> 
                <div><PlayerComponent id={props.id}/></div>
                <Link to={PLAYER_LIST_URL} className='App-Link' >
                <button>Go to List of Players page</button>
                </Link> 
            </header>
        </div>
    }
}

export default PlayerPage