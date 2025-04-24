import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Progress } from '@heroui/react';
import "./index.css"
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import Preview from './Preview';

function App() {
  const [downloadStatus, setDownloadStatus] = useState("")
  const [downloadProgress, setDownloadProgress] = useState(0);

  const [extractionStatus, setExtractionStatus] = useState("")
  const [extractionProgress, setExtractionProgress] = useState(0);
  const [done, setDone] = useState(false)

  useEffect(() => {
    let unlistenDownload: UnlistenFn | null;
    let unlistenExtract: UnlistenFn | null;
    let unlistenDone: UnlistenFn | null;
    listen<[number, string]>("download_status", (event) => {
      const [progress, status] = event.payload;
      setDownloadProgress(progress);
      setDownloadStatus(status);
    }).then(e => unlistenDownload = e)

    listen<[number, string]>("extraction_status", (event) => {
      const [progress, status] = event.payload;
      setExtractionProgress(progress);
      setExtractionStatus(status);
    }).then(e => unlistenExtract = e)

    listen("bootstrap_done", () => {
      setDone(true);
    }).then(e => unlistenDone = e)

    // This should do proper error handling
    invoke("bootstrap")
      .then(e => {
        console.log("Bootstrap done", e)
      })
      .catch((err) => {
        console.error(err)
      });


    return () => {
      if (unlistenDownload)
        unlistenDownload();
      if (unlistenExtract)
        unlistenExtract();
      if (unlistenDone)
        unlistenDone();
    }
  }, [])

  return (
    <main className="container w-full justify-center items-center flex flex-col">
      <h1>libobs - Example</h1>
      {!done &&
        <div className="flex gap-5 flex-col justify-center w-full items-center">
          <Progress
            aria-label="Loading..." className="max-w-md truncate"
            value={downloadProgress * 100}
            label={downloadStatus}
            showValueLabel
            disableAnimation
          />
          <Progress
            aria-label="Loading..." className="max-w-md truncate"
            value={extractionProgress * 100}
            label={extractionStatus}
            classNames={{
              indicator: "transition-transform duration-0"
            }}
            showValueLabel
            disableAnimation
          />
        </div>
      }
      {done && <Preview />}
      {/**TODO Add preview example here */}

    </main>
  );
}

export default App;
