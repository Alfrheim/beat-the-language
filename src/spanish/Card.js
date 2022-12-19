import React from 'react';

const Card = ({ word, translation, onClick, key}) => {
    const [isClicked, setIsClicked] = React.useState(false);
    const handleClick = () => {
        if(!isClicked) {
            setIsClicked(true);
        }
        onClick();
    }

    function isClickedAndCorrect() {
        console.log("is it clicked? {}", isClicked)
        console.log("is the correct word: {} {}", word, translation);
        if (isClicked){

            return word === translation ? '-correct': '-failed';
        }
        return '';
    }

    return (
        <div className="game-card" onClick = {handleClick}>
            <div className={`game-card2${isClickedAndCorrect()}`} >
                <h3>{word}</h3>
            </div>
        </div>
    );
};

export default Card;
