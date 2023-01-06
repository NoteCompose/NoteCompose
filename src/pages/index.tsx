import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";

function App() {
  useEffect(() => {
    listen("items", (windowEvent) => {
      if (typeof windowEvent.payload == "string") {
        const json = JSON.parse(windowEvent.payload);
        setItems(json);
      }
    }).then((unlisten) => {});
  }, []);

  const [items, setItems] = useState({
    file_name: null,
    items: [],
  });

  const svgItems = items.items.map((item) => {
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
    <div className="app">
      <ul className="tabs">
        <li>
          <div className="file-name">
            {items.file_name != null ? items.file_name : "Welcome"}
          </div>
          <div className="close">&#10006;</div>
        </li>
      </ul>

      <div className="content">
        {items.file_name != null ? (
          <svg>{svgItems}</svg>
        ) : (
          <div className="welcome">
            <h1>NoteCompose</h1>
            <h4>v0.1.0</h4>
            <div className="button">New</div>
            <div className="button">Open</div>
          </div>
        )}
      </div>
    </div>
  );
}

export default App;
