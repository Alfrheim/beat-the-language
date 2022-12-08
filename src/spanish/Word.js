import React from 'react';
import './word.css';

function Word({word}) {
    return (
        <div className="wordToTranslate">
            <h3>{word}</h3>
        </div>
    );
}

export default Word;