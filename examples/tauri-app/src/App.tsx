import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Progress, Card, Divider } from '@heroui/react';
import "./index.css"
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { LoadedPage } from './LoadedPage';

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

    console.log("Bootstrap started")
    // This should do proper error handling
    invoke("bootstrap")
      .then(e => {
        if(e === "OBS is already running." || e === "Done.")
          setDone(true)

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
    <main className="min-h-screen bg-gradient-to-b from-gray-50 to-gray-100 dark:from-gray-900 dark:to-gray-800 py-8 px-4">
      <div className="container mx-auto max-w-4xl">
        <Card className="shadow-xl">
          <div className="p-6">
            <div className="text-center mb-8">
              <h1 className="text-3xl font-bold text-gray-800 dark:text-white mb-2">libobs - Example</h1>
              <Divider className="my-4" />
              <p className="text-gray-600 dark:text-gray-300">
                {done ? "Setup complete! You can now use the application." : "Setting up required components..."}
              </p>
            </div>

            {!done && (
              <div className="space-y-8 my-10 max-w-2xl mx-auto">
                <div className="space-y-3">
                  <div className="flex justify-between text-sm font-medium">
                    <span>Downloading Components</span>
                    <span>{Math.round(downloadProgress * 100)}%</span>
                  </div>
                  <Progress
                    aria-label="Download Progress"
                    value={downloadProgress * 100}
                    className="h-2"
                    disableAnimation
                  />
                  <p className="text-sm text-gray-500 dark:text-gray-400">{downloadStatus}</p>
                </div>

                <div className="space-y-3">
                  <div className="flex justify-between text-sm font-medium">
                    <span>Extracting Files</span>
                    <span>{Math.round(extractionProgress * 100)}%</span>
                  </div>
                  <Progress
                    aria-label="Extraction Progress"
                    value={extractionProgress * 100}
                    className="h-2"
                    disableAnimation
                  />
                  <p className="text-sm text-gray-500 dark:text-gray-400">{extractionStatus}</p>
                </div>
              </div>
            )}

            {done && <LoadedPage />}
          </div>
        </Card>
      </div>
    </main>
  );
}

export default App;
