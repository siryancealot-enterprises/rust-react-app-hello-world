import React from 'react'
import { Link, useParams } from "react-router-dom";
import PlayerComponent from "../components/Player";
import { getPlayerListURL } from './PlayerListPage';
import { isNumericLiteral } from 'typescript';

export function getPlayerPageURL(number : number): string {
    return `/player/` + number;
}

export function getPlayerPageURLFormat(): string {
    return `/player/:number`;
}

const PlayerPage = () => { 
    const props = useParams();
    if  (props.number === undefined) {
        return <><div>Missing player "number" parameter in the URL!</div></> 
    } else {
        return <div className="App">
            <header className="App-header">
                <h2>Here's your player:</h2>
                <br/> 
                <div><PlayerComponent number={props.number}/></div>
                <Link to={getPlayerListURL()} className='App-Link' >
                <button>Go to List of Players page</button>
                </Link> 
            </header>
        </div>
    }
}

export default PlayerPage