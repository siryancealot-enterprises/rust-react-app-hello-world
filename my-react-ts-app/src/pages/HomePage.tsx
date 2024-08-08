import React from 'react';
import { Link } from "react-router-dom";
import logo from '../logo.svg';

const HomePage = () => {

    return  <div className="App">
      <header className="App-header">
       WELCOME TO PROJECT X!
        <img src={logo} className="App-logo" alt="logo" />
         <p>
           Edit <code>src/App.tsx</code> and save to reload.
         </p>
        <br/>
        <Link to={`user-list`} className='App-Link' >
            <button>Go to List of Users page</button>
        </Link> 
        </header>
    </div>

    //     <a
    //       className="App-link"
    //       href="https://veeva.com"
    //       target="_blank"
    //       rel="noopener noreferrer"
    //     >
    //       Learn About X
    //     </a>
    //     <a
    //       className="App-link"
    //       href="/rando?start=1&end=100"
    //       target="_blank"
    //       rel="noopener noreferrer"
    //     >
    //       Generate a random number
    //     </a>
    //     <div>List of users: <UsersInTheSystemComponent/></div>
        
    //     <BestTeamEverComponent/>
    //   </header>
    // </div>




        // <h1>Welcome to Project X!</h1>
        // <br/>
        // <Link to={`user-list`} className='App-Link' >
        //     <button>Go to List of Users page</button>
        // </Link> 
    // </div>


}

export default HomePage