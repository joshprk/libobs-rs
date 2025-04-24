import { invoke } from '@tauri-apps/api/core';
import { useEffect, useRef } from 'react';

export default function Preview() {
    const ref = useRef<HTMLDivElement>(null);

    useEffect(() => {
        if (ref.current) {
            const bounding = ref.current.getBoundingClientRect();
            invoke("add_preview", {
                x: bounding.x, y: bounding.y, width: bounding.width, height: bounding.height
            })
                .then(e => {
                    console.log("Preview added: ", e)
                })
        }

        const resizeHandler = () => {
            if (ref.current) {
                const bounding = ref.current.getBoundingClientRect();
                invoke("resize_preview", {
                    x: bounding.x, y: bounding.y, width: bounding.width, height: bounding.height
                })
                    .then(e => {
                        console.log("Preview updated: ", e)
                    })
            }
        }

        window.addEventListener("resize", resizeHandler);

        return () => {
            if (ref.current) {
                invoke("close_preview")
                    .then(e => {
                        console.log("Preview removed: ", e)
                    })
            }
            window.removeEventListener("resize", resizeHandler);
        }
    }, [])

    return <div ref={ref} className="w-full h-full aspect-video bg-gray-900 flex justify-center items-center">

    </div>
}