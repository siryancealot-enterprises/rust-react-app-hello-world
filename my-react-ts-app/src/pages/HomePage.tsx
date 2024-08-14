import React from 'react';
import { Link } from "react-router-dom";
import logo from '../logo.svg';
import { getPlayerListURL } from './PlayerListPage';
import { getPlayerCreateURL } from './PlayerCreatePage';

const HomePage = () => {

    return  <div className="App">
      <header className="App-header">
       WELCOME TO PROJECT X!
        <img src={logo} className="App-logo" alt="logo" />
        <Link to={getPlayerListURL()} className='App-Link' >
            <button>Go to List of Players page</button>
        </Link> 
        <Link to={getPlayerCreateURL()} className='App-Link' >
            <button>Add a player</button>
        </Link> 
        <Link to={`/rando?start=1&end=1000`} target="_blank" >
            <button>Generate a random number, eleswhere</button>
        </Link>
        <Link to={`/bad-url`} target="_blank" >
            <button>Go to a broken link</button>
        </Link>
        </header>
    </div>
}

export default HomePage