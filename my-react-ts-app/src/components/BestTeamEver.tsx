import React from "react";

const Logo = "https://logrocket-assets.io/static/home-hero-c97849b227a3d3015730e3371a76a7f0.svg";

const FirstComponent: React.FC<{}> = () => {
  return (
    <div>
        <h1>Los Angeles Lakers!</h1>
        <ul>
          <li>16 NBA Championships</li>
          <li>26 Hall of Famers</li>
          <li>5 top ten all time</li>
        </ul>
    </div>
  );
};
    
export default FirstComponent;