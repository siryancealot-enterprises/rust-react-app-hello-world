import { Route, Routes } from 'react-router-dom';
import './App.css';
import HomePage from './pages/HomePage';
import PageNotFoundPage from './pages/PageNotFoundPage';
import PlayerCreatePage, { getPlayerCreateURL } from './pages/PlayerCreatePage';
import PlayerListPage, { getPlayerListURL } from './pages/PlayerListPage';
import PlayerPage, { getPlayerPageURLFormat } from './pages/PlayerPage';


function App() {
  return (
    <>
      <Routes>
        <Route path="/" Component={HomePage} />
        <Route path={getPlayerPageURLFormat()} Component={PlayerPage} />
        <Route path={getPlayerListURL()} Component={PlayerListPage} />
        <Route path={getPlayerCreateURL()} Component={PlayerCreatePage} />
        <Route path="*" Component={PageNotFoundPage} />
      </Routes>
    </>
  );
}

export default App;
