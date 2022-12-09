import React from 'react';

const Card = ({ word, onClick }) => {
    return (
        <div className="game-card" onClick = {onClick}>
            <div className="game-card2" >
                <h3>{word}</h3>
            </div>
        </div>
    );
};

export default Card;
