import React from 'react';

const Card = ({ word, onClick }) => {
    return (
        <div className="card">
            <div className="face face1">
                <div className="content">
                    {word}
                </div>
            </div>
            <div className="face face2">
                <div className="content">
                    <button type="button" onClick={onClick}>Read More</button>
                </div>
            </div>
        </div>
    );
};

export default Card;
