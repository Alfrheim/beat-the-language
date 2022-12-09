import React from "react";
import './score.css';

function Score({score : [ wins, ties, losses]} ) {

  return (
    <div className="score">
        <div className="perfect">{wins}</div>
        <div className="good">{ties}</div>
        <div className="failed">{losses}</div>
    </div>
  );
}
export default Score