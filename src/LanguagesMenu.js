import React from 'react';
import {Link} from "react-router-dom";

function LanguagesMenu() {

    return (
        <div className="container">
            <Link to="/spanish" state={{cmd:'get_word'}}>Learn spanish words</Link>
            <Link to="/spanish" state={{cmd:'get_verb'}}>Learn spanish words</Link>
        </div>
    );
}

export default LanguagesMenu;
