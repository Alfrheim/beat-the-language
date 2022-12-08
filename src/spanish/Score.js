import React from "react";
import './score.css';

function Score({ wins = <>0</>, ties = <>0</>, losses = <>0</>} ) {

  return (
    <div className="score">
        <div className="perfect">{wins}</div>
        <div className="good">{ties}</div>
        <div className="failed">{losses}</div>
    </div>
  );
}
export default Score