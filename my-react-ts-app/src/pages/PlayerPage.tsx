import React from 'react'
import { Link, useParams } from "react-router-dom";
import PlayerComponent from "../components/Player";
import { getPlayerListURL } from './PlayerListPage';

export function getPlayerPageURL(id : string | null): string {
    return `/player/` + id;
}

export function getPlayerPageURLFormat(): string {
    return `/player/:id`;
}

const PlayerPage = () => { 
    const props = useParams();
    if  (props.id === undefined) {
        return <><div>Missing player "id" in the URL!</div></> 
    } else {
        return <div className="App">
            <header className="App-header">
                <h2>Here's your player:</h2>
                <br/> 
                <div><PlayerComponent id={props.id}/></div>
                <Link to={getPlayerListURL()} className='App-Link' >
                <button>Go to List of Players page</button>
                </Link> 
            </header>
        </div>
    }
}

export default PlayerPage