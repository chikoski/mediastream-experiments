import type { Size } from "./model/size";
import { pngGlitchable } from "./png-glitchable/png-glitch"

console.log("from worker");

// biome-ignore lint/suspicious/noGlobalAssign: to set onmessage callback
onmessage = async (message) => {
	if (message.data.command === "start") {
		const { readable, writable, size } = message.data;
		const ve = new GlitchFilter(size);
		readable.pipeThrough(ve.filter).pipeTo(writable);
	}
};

class GlitchFilter {
	filter: TransformStream;
	private canvas: OffscreenCanvas;
	private gc: OffscreenCanvasRenderingContext2D | null;

	private width: number;
	private height: number;

	constructor({ width, height }: Size) {
		this.width = width;
		this.height = height;

		this.canvas = new OffscreenCanvas(width, height);
		this.gc = this.canvas.getContext("2d");
		this.filter = new TransformStream(this);
	}

    transform(
        frame: VideoFrame,
		conteroller: TransformStreamDefaultController<VideoFrame>,
    ) {
        if(this.gc != null) {
            this.doTransform(frame, conteroller, this.gc);
        }else {
            conteroller.enqueue(frame);
        }
    }

	private async doTransform(
		frame: VideoFrame,
		conteroller: TransformStreamDefaultController<VideoFrame>,
		gc: OffscreenCanvasRenderingContext2D,
	) {
		gc.drawImage(frame, 0, 0);
		const copyTimes = Math.floor(10 * Math.random());
		for (let i = 0; i < copyTimes; i++) {
			const src = Math.floor(Math.random() * (this.height - 50));
			const dest = Math.floor(Math.random() * (this.height - 50));
			gc.drawImage(
				this.canvas,
				0,
				src,
				this.width,
				50,
				0,
				dest,
				this.width,
				50,
			);
		}

        const imageData = gc.getImageData(0, 0, this.width, this.height);
        const buffer = new Uint8Array(imageData.data);
        
        const glitcher = await pngGlitchable.Png.create(buffer, this.width, this.height);
        
        glitcher.getScanLines().forEach((scanLine, index) => {
            if(index % 351 < 40) {
                scanLine.setFilterType("paeth");
            }
        });
        
        const glitched = glitcher.read();
        const bitmap = await createImageFromPng(glitched);
        const videoFrame = new VideoFrame(bitmap, { timestamp: frame.timestamp });
        frame.close();
        conteroller.enqueue(videoFrame);
	}
}

function createImageFromPng(data: Uint8Array): Promise<ImageBitmap> {
    const blob = new Blob([data], {type: "image/png"});
    return createImageBitmap(blob);
}