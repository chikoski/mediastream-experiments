import { useState, type ReactNode } from "react";
import { DefaultUiState, StreamOpened, type UiState } from "./ui-state";
import DefaultScreen from "./default-screen";
import MediaStreamOpenedScreen from "./stream-opended-screen";

type AppProps = {
	worker: Worker;
};

export default function App({ worker }: Readonly<AppProps>): ReactNode {
	const [uiState, updateUiState] = useState<UiState>(new DefaultUiState());

	const requestMediaStream = async () => {
		console.log("Reqesting a MediaStream");
		const stream = await navigator.mediaDevices.getUserMedia({
			video: {
				width: 1920,
				height: 1080,
			},
			audio: false,
		});
		const newState = new StreamOpened(stream, worker);
		console.log(newState);
		updateUiState(newState);
	};

	const stopMediaStream = () => {
		console.log("Stopping a MediaStream");
		if (uiState instanceof StreamOpened) {
			uiState.stopStream();
		}
		updateUiState(new DefaultUiState());
	};

	if (uiState instanceof StreamOpened) {
		return (
			<MediaStreamOpenedScreen
				input={uiState.output}
				stopMediaStream={stopMediaStream}
			/>
		);
	}

	return <DefaultScreen requestMediaStream={requestMediaStream} />;
}
