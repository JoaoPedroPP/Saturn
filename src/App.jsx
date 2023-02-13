import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [state, setState] = useState("Start");
  const [marca, setMarca] = useState("Marcar");

  const mark = async () => {
    const date = new Date();
    setMarca(await invoke("mark", { state: state, time: date.toISOString() }));
    setState(state === "Start" ? "End" : "Start");
    setTimeout(() => {
      setMarca("Marcar");
    }, 1500);
  };

  return (
    <div className="container">
      <h3>Ponto</h3>
      <button onClick={mark}>{marca}</button>
    </div>
  );
}

export default App;
