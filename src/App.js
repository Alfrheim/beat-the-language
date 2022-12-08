import {createBrowserRouter, RouterProvider} from "react-router-dom";
import Home from "./Home";
import Spanish from "./spanish/Spanish";

const router = createBrowserRouter([
    {path: '/', element: <Home/>},
  {path: '/spanish', element: <Spanish/>},
])
function App() {
  return (
        <RouterProvider router={router}/>
  );

}

export default App;
