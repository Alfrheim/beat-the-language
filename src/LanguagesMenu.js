import React from 'react';
import { Link } from "react-router-dom";
import './language-menu.css';

function LanguagesMenu() {

  return (
    <div className="container">
      <Link to="/spanish" state={{ cmd: 'get_word' }}><button className="button menu" type="button"><h3>WORDS</h3></button></Link>
      <Link to="/spanish" state={{ cmd: 'get_verb' }}><button className="button menu" type="button"><h3>VERBS</h3></button></Link>
    </div>
  );
}

export default LanguagesMenu;
