import React, { useLayoutEffect, useRef } from "react";
import { gsap } from "gsap";
import "./splashscreen.css";
import {Helmet} from "react-helmet"
import { useNavigate  } from 'react-router-dom';

function SplashScreen() {
    const root = useRef();

    useLayoutEffect(() => {
        let ctx = gsap.context(() => {
            gsap
                .timeline({ repeat: 10, repeatDelay: 1 })
                .from(".mask div", {
                    xPercent: gsap.utils.wrap([100, -100]),
                    stagger: 0.4,
                    opacity: 0,
                    ease: "circ.inOut"
                })
                .to(
                    ".mask div",
                    {
                        opacity: 0,
                        yPercent: gsap.utils.wrap([-100, 100]),
                        duration: 1,
                        ease:"none"
                    },
                    ">0.5"
                );

            gsap.fromTo(".bar", {x:-200}, {x:200, duration:20, ease:"none", repeat:3, yoyo:true})
            // all your animations go in here...
        }, root); // <- scopes all selector text to the root element

        return () => ctx.revert();
    }, []);
    const navigate = useNavigate();
    const navigateTo = () => {
        navigate("/menu");
    }
    return (
        <>
        <Helmet>
            {/*<link rel="preconnect" href="https://fonts.googleapis.com"/>*/}
            {/*<link rel="preconnect" href="https://fonts.gstatic.com" crossOrigin/>*/}
            <link
                href="https://fonts.googleapis.com/css2?family=Kanit:wght@200;300;400&family=Russo+One&display=swap"
                rel="stylesheet"/>

        </Helmet>

            <div ref={root} className="contentWrapper" onClick={navigateTo}>
                <div className="bg"></div>
                <div className="barWrapper">
                    <div className="bar"></div>
                </div>
                <div className="contentWrapper">
                    <div className="mainContent flexCenter">
                        <div className="maskOut getCreative">
                            <div className="mask">
                                <div><span className="thin">BEAT</span></div>
                            </div>
                            <div className="mask">
                                <div>LANGUAGE</div>
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