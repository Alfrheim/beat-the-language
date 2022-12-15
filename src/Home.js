import './home.css';
import LanguagesMenu from "./LanguagesMenu";
import SplashScreen from "./SplashScreen";
import LinkTo from "./Utils";

function Home() {
  return (
    <div className="App" onClick={() =>
        <LinkTo link="/spanish" />
    }>
      <header className="App-header">
      </header>
      {/*<LanguagesMenu />*/}
        <SplashScreen />
    </div>
  );
}

export default Home;
