import React from 'react'
import { Link } from "react-router-dom";
import UsersInTheSystemComponent from "../components/UesrList";
import BestTeamEver from "../components/BestTeamEver";

const UserListPage = () => {

    return <div className="App">
        <header className="App-header">
            <div><BestTeamEver /></div>
            <div>List of users: <UsersInTheSystemComponent/></div>
            <Link to={`/`} >
                <button>Go to home</button>
            </Link>
        </header>
    </div>

}

export default UserListPage