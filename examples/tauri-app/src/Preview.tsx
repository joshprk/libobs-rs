import { invoke } from '@tauri-apps/api/core';
import { useEffect, useRef } from 'react';

export default function Preview() {
    const ref = useRef<HTMLDivElement>(null);

    useEffect(() => {
        let wasAdded = false;

        if (ref.current) {
            const bounding = ref.current.getBoundingClientRect();
            let x = Math.round(bounding.x);
            let y = Math.round(bounding.y);
            let width = Math.round(bounding.width);
            let height = Math.round(bounding.height);
            invoke("add_preview", {
                x, y, width, height
            })
                .then(e => {
                    wasAdded = true;
                    console.log("Preview added: ", e)
                })
        }

        const resizeHandler = () => {
            if (ref.current && wasAdded) {
                const bounding = ref.current.getBoundingClientRect();
                let x = Math.round(bounding.x);
                let y = Math.round(bounding.y);
                let width = Math.round(bounding.width);
                let height = Math.round(bounding.height);
                console.log(bounding.top)
                invoke("resize_preview", {
                    x, y, width, height
                })
                    .then(e => {
                        console.log("Preview updated: ", e)
                    })
            }
        }

        window.addEventListener("resize", resizeHandler);
        window.addEventListener("scroll", resizeHandler);

        return () => {
            if (!wasAdded) return;

            if (ref.current) {
                console.log("Removing preview")
                invoke("close_preview")
                    .then(e => {
                        console.log("Preview removed: ", e)
                    })
            }
            window.removeEventListener("resize", resizeHandler);
            window.removeEventListener("scroll", resizeHandler);
        }
    }, [])

    return <div ref={ref} className="w-full h-full aspect-video bg-gray-900 flex justify-center items-center">

    </div>
}