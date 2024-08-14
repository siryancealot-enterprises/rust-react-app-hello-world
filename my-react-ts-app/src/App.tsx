import React from 'react';
import { Routes, Route } from 'react-router-dom';
import './App.css';
import HomePage from './pages/HomePage';
import PlayerPage, { getPlayerPageURLFormat } from './pages/PlayerPage';
import PlayerListPage, { getPlayerListURL } from './pages/PlayerListPage';
import PlayerCreatePage, { getPlayerCreateURL } from './pages/PlayerCreatePage';
import PageNotFoundPage from './pages/PageNotFoundPage';


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
