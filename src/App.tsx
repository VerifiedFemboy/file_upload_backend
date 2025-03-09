import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import Home from "./pages/Home";
import SexyPage from "./pages/SexyPage";

export default function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/nigger1/nigger2/nigger3/sexypage" element={<SexyPage />} />
      </Routes>
    </Router> 
  )
}