"use client";

import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

export default function Greet() {
  useEffect(() => {
    invoke<string>("greet", { name: "Next.js" })
      .then(console.log)
      .catch(console.error);
  }, []);

  // Necessary because we will have to use Greet as a component later.
  return (
    <div className="flex gap-4">
      <button
        className="btn btn-neutral"
        onClick={() => {
          invoke("create_volume");
        }}
      >
        创建加密映像
      </button>
      <button
        className="btn btn-secondary"
        onClick={() => invoke("load_volume")}
      >
        挂载加密映像
      </button>
    </div>
  );
}
