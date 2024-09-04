import { Link } from "react-router-dom";
import PlayerSearchComponent, { SearchTerm } from '../components/PlayerSearch';
import { PLAYER_LIST_URL } from './PlayerListPage';

export const PLAYER_SEARCH_URL = `/player-search`;


function PlayerSearchPage() {
  function handleSubmit(formData: SearchTerm) {
    console.log(formData);
  }

  return (
    <div className="App">
      <header className="App-header">
        <div><PlayerSearchComponent onSubmit={handleSubmit} /></div>
        <br />
        <div>
          <Link to={PLAYER_LIST_URL} ><button>Go to List of Players</button></Link>
        </div>
      </header>
    </div>
  );

}

export default PlayerSearchPage;