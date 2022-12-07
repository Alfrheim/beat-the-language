import React from 'react';
import LinkTo from "./Utils";
function Card({ languageTitle, onClick, readMoreText, onClickText}) {
    return (
        <div className="card">
            <div className="face face1">
                <div className="content">
                    <i>{languageTitle}</i>
                    <h3>descripcion en espanol de lo que significa la definicion superior</h3>
                </div>
            </div>
            <div className="face face2">
                <div className="content">
                    <p>{readMoreText}</p>
                    <LinkTo link={onClick} message={onClickText}/>
                </div>
            </div>
        </div>
    );
}

export default Card;
