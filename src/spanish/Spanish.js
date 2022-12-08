import React from 'react';
import "./spanish.css";
import CardComponent from "./CardComponent";
import Score from "./Score";
import Word from "./Word";

function Spanish() {
    return (
        <div className="cards">
            <Score />
            <Word word='House'/>
            <CardComponent />
            {/*// <!-- footer -->*/}
        </div>
    );
}

export default Spanish;