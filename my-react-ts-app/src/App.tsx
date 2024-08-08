import React from 'react';
import { Routes, Route } from 'react-router-dom';
import './App.css';
import HomePage from './pages/HomePage';
import UserListPage from './pages/UserListPage';


function App() {
  return (
    <>
      <Routes>
        <Route path="/" Component={HomePage} />
        <Route path="/user-list" Component={UserListPage} />
      </Routes>
    </>
  );
}

export default App;
