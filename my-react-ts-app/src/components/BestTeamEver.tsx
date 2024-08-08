import React from "react";

const Logo = "https://logrocket-assets.io/static/home-hero-c97849b227a3d3015730e3371a76a7f0.svg";

const FirstComponent: React.FC<{}> = () => {
  return (
    <div>
        <h1>Los Angeles Lakers!</h1>
        <p>16 NBA Championships</p>
        <p>10 Hall of Famers</p>
        <p>5 top ten all time</p>
        <p>NUFF SAID!</p>
        <a href="/">Go Back</a>
    </div>
  );
};
    
export default FirstComponent;