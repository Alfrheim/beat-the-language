import React from 'react';
import {Helmet} from "react-helmet"

function SplashScreen() {

    return (
        <>
        <Helmet>
            <script src="./GetScript.js" type="text/javascript"/>
            <link rel="preconnect" href="https://fonts.googleapis.com"/>
            <link rel="preconnect" href="https://fonts.gstatic.com" crossOrigin/>
            <link
                href="https://fonts.googleapis.com/css2?family=Kanit:wght@200;300;400&family=Russo+One&display=swap"
                rel="stylesheet"/>

        </Helmet>
            <div className="contentWrapper">
                <div className="bg"></div>
                <div className="barWrapper">
                    <div className="bar"></div>
                </div>
                <div className="contentWrapper">
                    <div className="mainContent flexCenter">
                        <div className="maskOut getCreative">
                            <div className="mask">
                                <div><span className="thin">GET</span></div>
                            </div>
                            <div className="mask">
                                <div>CREATIVE</div>
                            </div>
                        </div>
                        <div className="maskOut haveFun">
                            <div className="mask">
                                <div><span className="thin">HAVE</span></div>
                            </div>
                            <div className="mask">
                                <div>FUN</div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </>
    );
}

export default SplashScreen;