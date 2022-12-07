import './style.css';
import LanguagesMenu from "./LanguagesMenu";
import Spanish from "./spanish/Spanish";
import {Routes, Route} from 'react-router-dom';

function App() {
  return (
    <div className="App">
      <header className="App-header">
      </header>
      <LanguagesMenu />
        <Routes>
            <Route path="/spanish" component={Spanish}/>
            <Route exact path="/" component={App}/>
        </Routes>
    </div>
  );
}

export default App;
