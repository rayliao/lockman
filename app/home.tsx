"use client";

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import {
  BeakerIcon,
  LockClosedIcon,
  LockOpenIcon,
} from "@heroicons/react/24/outline";

export default function Home() {
  const [list, setList] = useState<string[]>();
  useEffect(() => {
    invoke<string[]>("list_volume").then((res) => setList(res));
  }, []);

  return (
    <div className="flex justify-center">
      {list && list.length ? (
        list.map((item, index) => (
          <div
            key={index}
            className="swap swap-rotate p-2 rounded-full bg-neutral text-neutral-content hover:bg-neutral/95"
          >
            <div className="flex flex-col w-60 h-60 px-5 py-2 gap-2 justify-center items-center border border-info rounded-full">
              <div className="w-10 h-10 relative">
                <LockOpenIcon className="absolute swap-on h-full w-full text-primary" />
                <LockClosedIcon className="absolute swap-off h-full w-full" />
              </div>
              <p>{item}</p>
            </div>
          </div>
        ))
      ) : (
        <>
          <button
            className="btn btn-neutral"
            onClick={() => {
              invoke("create_volume", { name: "" });
            }}
          >
            <BeakerIcon className="h-6 w-6 text-blue-500" />
            创建加密映像
          </button>
          <button
            className="btn btn-secondary"
            onClick={() => invoke("load_volume")}
          >
            挂载加密映像
          </button>
        </>
      )}
    </div>
  );
}
