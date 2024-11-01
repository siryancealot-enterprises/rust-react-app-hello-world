import { Link } from "react-router-dom";
import logo from '../logo.svg';
import { PLAYER_CREATE_URL } from './PlayerCreatePage';
import { PLAYER_LIST_URL } from './PlayerListPage';
import { PLAYER_SEARCH_URL } from './PlayerSearchPage';

const HomePage = () => {

    return <div className="App">
        <header className="App-header">
            WELCOME TO PROJECT X!
            <img src={logo} className="App-logo" alt="logo" />
            <Link to={PLAYER_LIST_URL} >
                <button>Go to List of Players page</button>
            </Link>
            <Link to={PLAYER_SEARCH_URL} >
                <button>Go to Search Players page</button>
            </Link>
            <Link to={PLAYER_CREATE_URL} >
                <button>Add a player</button>
            </Link>
            <Link to={`/bad-url`} target="_blank" >
                <button>Go to a broken link</button>
            </Link>
        </header>
    </div>
}

export default HomePage