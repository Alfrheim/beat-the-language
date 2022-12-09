import React from 'react';
import Card from './Card';

function CardComponent({cards, onClick}) {
    return (
        <div className="container">
            {cards.map((card)=>
                <Card
                    word = {card}
                    readMoreText = "Learn More"
                    onClick = {onClick(card)}
                    key = {card}
                />
            )}
        </div>
    );
};

export default CardComponent;
