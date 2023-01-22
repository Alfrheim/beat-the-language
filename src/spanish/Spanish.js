import React, {useEffect, useState} from 'react';
import "./spanish.css";
import CardComponent from "./CardComponent";
import Score from "./Score";
import Word from "./Word";
import { invoke } from '@tauri-apps/api';
import {useLocation} from "react-router-dom";

function GetEnglishRandomWord(cmd) {
const [state, setState] = useState({
    word: '',
    translation: '',
    choices: [],
    selected: []
})
    function promise() {
        getRandomWord(cmd).then((res) =>
            setState({ ...state, word: res.word, translation: res.translation ,choices: res.choices})
        );
    }

    useEffect(promise, [])

    return [state, setState]
}

function getRandomWord(cmd = 'get_verb') {
    return invoke(cmd, {language: "EN"})
}

function Spanish() {
    const location = useLocation();
    const { cmd } = location.state;
    let command = cmd;
    const [state, setState] = GetEnglishRandomWord(command)
    const [score, setScore] = useState([0, 0, 0])
    return (
        <div className="cards" onClick={
            ()=> {
                if (state.selected.includes(state.translation) || state.selected.length === 2)
            getRandomWord(cmd).then((res) =>
                setState({ ...state, word: res.word, translation: res.translation, selected: [] ,choices: res.choices})
            );
            console.log(state.choices)
        }}>
            <Score score = {score}/>
            <Word word={state.word}/>
            <CardComponent cards={state.choices}
                           translation = {state.translation}
                           onClick= {
                (value) => () => {
                    const failed = state.selected.length;
                    setState({...state, selected: [...state.selected, value]});
                    if (value === state.translation) {
                        if(failed === 0) {
                            setScore([++score[0], score[1], score[2]])
                            return
                        }
                        if(failed === 1 && !state.selected.includes(state.translation)) {
                            setScore([score[0], ++score[1], score[2]])
                            return
                        }
                    }

                    if(failed === 1 && !state.selected.includes(state.translation))
                        setScore([score[0], score[1], ++score[2]])
                }
            }/>
            {/*// <!-- footer -->*/}
        </div>
    );
}

export default Spanish;