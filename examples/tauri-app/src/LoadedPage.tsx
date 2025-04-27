import { Button } from '@heroui/react';
import Preview from './Preview';
import { invoke } from '@tauri-apps/api/core';

export function LoadedPage() {
    return <div className="flex flex-col gap-5 justify-center items-center w-full">
        <Preview />
        <Button onPress={() => invoke("switch_monitor")}>Switch monitor</Button>
    </div>
}