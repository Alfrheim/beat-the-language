import React from 'react';
import Card from './Card';

function CardComponent() {
    return (
        <div className="container">
            <Card
                word="Casa"
                readMoreText="Learn More"
                onClick={() => alert('Learn More button for card 1 was clicked')}
            />
            <Card
                word="Papelera"
                readMoreText="More Info"
                onClick={() => alert('More Info button for card 2 was clicked')}
            />
            <Card
                word="Electrodomestico"
                readMoreText="More Info"
                onClick={() => alert('More Info button for card 2 was clicked')}
            />
        </div>
    );
};

export default CardComponent;
