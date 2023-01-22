import React from 'react';
import Card from './Card';

function CardComponent({cards, translation, onClick}) {
    return (
        <div className="container">
            {cards.map((card)=>
                <Card
                    word = {card}
                    readMoreText = "Learn More"
                    translation = {translation}
                    onClick = {onClick(card)}
                    key = {card}
                />
            )}
        </div>
    );
};

export default CardComponent;
