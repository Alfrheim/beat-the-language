import React, {useEffect, useState} from 'react';
import "./spanish.css";
import CardComponent from "./CardComponent";
import Score from "./Score";
import Word from "./Word";
import { invoke } from '@tauri-apps/api';

function GetEnglishRandomWord() {
    const [randomWord, setWord] = useState('')

    function getRandomWord() {
        invoke('get_random_word', {language: "EN"}).then((message) => setWord(message))
    }
    useEffect(getRandomWord, [])

    return {randomWord}
}

function Spanish() {
    const {randomWord} = GetEnglishRandomWord()
    return (
        <div className="cards">
            <Score />
            <Word word={randomWord}/>
            <CardComponent />
            {/*// <!-- footer -->*/}
        </div>
    );
}

export default Spanish;