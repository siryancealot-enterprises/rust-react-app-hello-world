import React from 'react';
import { Routes, Route } from 'react-router-dom';
import './App.css';
import HomePage from './pages/HomePage';
import PlayerListPage from './pages/PlayerListPage';
import PlayerCreatePage from './pages/PlayerCreatePage';
import PageNotFoundPage from './pages/PageNotFoundPage';


function App() {
  return (
    <>
      <Routes>
        <Route path="/" Component={HomePage} />
        <Route path="/player-list" Component={PlayerListPage} />
        <Route path="/player-create" Component={PlayerCreatePage} />
        <Route path="*" Component={PageNotFoundPage} />
      </Routes>
    </>
  );
}

export default App;
