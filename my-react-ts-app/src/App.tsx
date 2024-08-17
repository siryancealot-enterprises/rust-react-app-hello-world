import { Route, Routes } from 'react-router-dom';
import './App.css';
import HomePage from './pages/HomePage';
import PageNotFoundPage from './pages/PageNotFoundPage';
import PlayerCreatePage, { PLAYER_CREATE_URL } from './pages/PlayerCreatePage';
import PlayerListPage, { PLAYER_LIST_URL } from './pages/PlayerListPage';
import PlayerPage, { PLAYER_DETAIL_URL_FORMAT } from './pages/PlayerPage';


function App() {
  return (
    <>
      <Routes>
        <Route path="/" Component={HomePage} />
        <Route path={PLAYER_DETAIL_URL_FORMAT} Component={PlayerPage} />
        <Route path={PLAYER_LIST_URL} Component={PlayerListPage} />
        <Route path={PLAYER_CREATE_URL} Component={PlayerCreatePage} />
        <Route path="*" Component={PageNotFoundPage} />
      </Routes>
    </>
  );
}

export default App;
