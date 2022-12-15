import { gsap } from "gsap";

const tl = gsap
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