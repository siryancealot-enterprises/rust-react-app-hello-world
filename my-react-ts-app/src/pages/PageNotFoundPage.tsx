import { Link } from "react-router-dom";
import logo from '../logo.svg';

const PageNotFoundPage = () => {

    return  <div className="App">
      <header className="App-header">
       <h1>UH OH, YOU'VE HIT THE 404!</h1>
       <p>Please check the URL, and tell us if you think it needs fixing!</p> 
        <img src={logo} className="App-logo" alt="logo" />
        <Link to={`/`} >
            <button>Go to home</button>
        </Link>
        </header>
    </div>
}

export default PageNotFoundPage