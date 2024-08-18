import { Link } from "react-router-dom";
import { Player } from '../components/Player';
import PlayerCreateComponent from '../components/PlayerCreate';
import { PLAYER_LIST_URL } from './PlayerListPage';

export const PLAYER_CREATE_URL = `/player-create`;


function PlayerCreatePage() {
  function handleSubmit(formData: Player) {
    console.log(formData);
  }

  return (
    <div className="App">
      <header className="App-header">
        <div><PlayerCreateComponent onSubmit={handleSubmit} /></div>
        <br />
        <div>
          <Link to={PLAYER_LIST_URL} ><button>Go to List of Players</button></Link>
        </div>
      </header>
    </div>
  );

}

export default PlayerCreatePage;