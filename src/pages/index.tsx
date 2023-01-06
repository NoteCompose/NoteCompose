import { useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { emit, listen } from "@tauri-apps/api/event";
import Image from "next/image";
import reactLogo from "../assets/react.svg";
import tauriLogo from "../assets/tauri.svg";
import nextLogo from "../assets/next.svg";

function App() {
  const [greetMsg, setGreetMsg] = useState("");

  useEffect(() => {
    listen("items", (windowEvent) => {
      if (typeof windowEvent.payload == "string") {
        const json = JSON.parse(windowEvent.payload);
        setItems(json);
      }
    }).then((unlisten) => {});
  }, []);

  const [items, setItems] = useState([]);

  const svgItems = items.map((item) => {
    console.log(item);
    if (item.kind == "line") {
      return (
        <line
          x1={item.element.x1}
          y1={item.element.y1}
          x2={item.element.x2}
          y2={item.element.y2}
        />
      );
    } else if (item.kind == "path") {
      return <path d={item.element} />;
    }
  });

  return (
    <div>
      <h1>{greetMsg}</h1>
      <svg>{svgItems}</svg>
    </div>
  );
}

export default App;
