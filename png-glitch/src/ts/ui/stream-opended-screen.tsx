import { type ReactNode, useEffect, useRef } from "react";
import { PrimaryButton } from "./component/button";

type MediaStreamOpenedScreenProps = {
	input: MediaStream;
	stopMediaStream: () => void;
};

export default function MediaStreamOpenedScreen({
	input,
	stopMediaStream,
}: Readonly<MediaStreamOpenedScreenProps>): ReactNode {
	const player = useRef<HTMLVideoElement>(null);
	useEffect(() => {
        console.log("Setting the acquired MediaStream to the Video element");
		const p = player.current;
		if (p != null) {
			p.srcObject = input;
            p.play();
		}
	});

	return (
		<div>
			<video ref={player} width={1920} height={1080} />
			<PrimaryButton onClick={stopMediaStream}>Stop</PrimaryButton>
		</div>
	);
}
