import React, {useEffect, useState} from 'react';
import "./spanish.css";
import CardComponent from "./CardComponent";
import Score from "./Score";
import Word from "./Word";
import { invoke } from '@tauri-apps/api';

function GetEnglishRandomWord() {
const [state, setState] = useState({
    word: '',
    translation: '',
    choices: [],
    selected: []
})
    function getRandomWord() {
        //{ word , tranlation, choices[], selected [] }
        invoke('get_random_word', {language: "EN"}).then((message) =>
            setState({ ...state, word: message, translation:"Casa" ,choices: [ "Casa", "Violin", "Electrodomestico"]})
        );
        // setState({...state, choices: ["Violin", "Electrodomestico"]});
}

    useEffect(getRandomWord, [])

    return [state, setState]
}

function Spanish() {
    const [state, setState] = GetEnglishRandomWord()
    const [score, setScore] = useState([0, 0, 0])
    return (
        <div className="cards">
            <Score score = {score}/>
            <Word word={state.word}/>
            <CardComponent cards={state.choices}
                           onClick= {
                (value) => () => {
                    const failed = state.selected.length;
                    setState({...state, selected: [...state.selected, value]});
                    if (value === state.translation) {
                        console.log(failed)
                        if(failed === 0) {
                            setScore([++score[0], score[1], score[2]])
                            return
                        }
                        if(failed === 1) {
                            setScore([score[0], ++score[1], score[2]])
                            return
                        }
                    }

                    if(failed === 1)
                        setScore([score[0], score[1], ++score[2]])
                }
            }/>
            {/*// <!-- footer -->*/}
        </div>
    );
}

export default Spanish;