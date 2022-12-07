import React from 'react';
import { useNavigate  } from 'react-router-dom';

// function LinkTo({link, message}) {
//     return <Link to={link}>{message}</Link>
// }
function LinkTo({link, message}) {
    const navigate = useNavigate();
    const navigateTo = () => {
        navigate(link);
    }
    return <button onClick={navigateTo} type='button'>{message}</button>
}

export default LinkTo;