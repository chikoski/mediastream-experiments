import { Size } from "../model/size";

export type UiState = DefaultUiState | StreamOpened;

export class DefaultUiState {}

export class StreamOpened {
	private worker: Worker;
	private input: MediaStream;
	private videoTrack: MediaStreamVideoTrack;
	private processor: MediaStreamTrackProcessor<VideoFrame>;
	private generator: MediaStreamTrackGenerator<VideoFrame>;
	output: MediaStream;

	constructor(input: MediaStream, worker: Worker) {
		this.input = input;
		const [videoTrack] = input.getVideoTracks();
		this.videoTrack = videoTrack;
		this.worker = worker;

		this.processor = new MediaStreamTrackProcessor({ track: this.videoTrack });
		this.generator = new MediaStreamTrackGenerator({ kind: "video" });

		const { readable } = this.processor;
		const { writable } = this.generator;
        const size = Size.create(videoTrack);

		worker.postMessage({ command: "start", readable, writable, size }, [
			readable,
			writable,
		]);

		this.output = new MediaStream([this.generator]);
	}

	addEventListener(
		eventName: keyof MediaStreamTrackEventMap,
		handler: () => void,
	) {
		this.videoTrack.addEventListener(eventName, handler);
	}

	stopStream(): void {
		this.videoTrack.stop();
	}
}


