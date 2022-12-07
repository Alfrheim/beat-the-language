import React from 'react';
import Card from './Card';

function LanguagesMenu() {

    return (
        <div className="container">
            <Card
                languageTitle="Spanish"
                readMoreText="Learn More"
                onClick="/spanish"
                onClickText="Learn Spanish"
            />
            <Card
                languageTitle="German"
                readMoreText="More Info"
                onClick="/spanish"
                onClickText="Learn German"
            />
        </div>
    );
}

export default LanguagesMenu;
