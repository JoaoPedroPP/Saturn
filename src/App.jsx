import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [marca, setMarca] = useState("Marcar");

  const mark = async () => {
    setMarca("Marcado!");
    setTimeout(() => {
      setMarca("Marcar");
    }, 1500);
  };

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="container">
      <h3>Ponto</h3>
      <button onClick={mark}>{marca}</button>
    </div>
  );
}

export default App;
