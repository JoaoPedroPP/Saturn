import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

import { copyFile } from "@tauri-apps/api/fs";

import { downloadDir, homeDir } from "@tauri-apps/api/path";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faFileExport } from "@fortawesome/free-solid-svg-icons";

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

  const exportReport = async () => {
    try {
      const home = await homeDir();
      const download = await downloadDir();
      await copyFile(`${home}.ponto/ponto.csv`, `${download}report.csv`);
      await invoke("open_file_path", { path: `${download}report.csv` });
    } catch (error) {
      console.log(error);
    }
  };

  return (
    <div className="container">
      <div className="title">
        <h3>Ponto</h3>
        <FontAwesomeIcon
          onClick={exportReport}
          className="icon"
          icon={faFileExport}
        />
      </div>
      <button onClick={mark}>{marca}</button>
    </div>
  );
}

export default App;
