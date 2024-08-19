import { Link, useParams } from "react-router-dom";
import PlayerComponent from "../components/Player";
import { PLAYER_LIST_URL } from './PlayerListPage';

export function getPlayerDetailURL(id: string | undefined): string {
    if (id === undefined) {
        throw new Error('Missing id');
    }
    return `/player/` + id;
}

export const PLAYER_DETAIL_URL_FORMAT = `/player/:id`;


function PlayerPage() {
    const props = useParams();
    if (props.id === undefined) {
        return <><div>Missing player "id" in the URL!</div></>
    } else {
        return <div className="App">
            <header className="App-header">
                <h2>Here's your player:</h2>
                <br />
                <div><PlayerComponent id={props.id} /></div>
                <br />
                <br />
                <Link to={PLAYER_LIST_URL} >
                    <button>Go to List of Players page</button>
                </Link>
            </header>
        </div>
    }
}

export default PlayerPage