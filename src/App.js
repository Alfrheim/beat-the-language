import {createBrowserRouter, RouterProvider} from "react-router-dom";
import Home from "./Home";
import Spanish from "./spanish/Spanish";
import LanguagesMenu from "./LanguagesMenu";

const router = createBrowserRouter([
    {path: '/', element: <Home/>},
    {path: '/spanish', element: <Spanish/>},
    {path: '/menu', element: <LanguagesMenu/>},
])
function App() {
  return (
        <RouterProvider router={router}/>
  );

}

export default App;
