import type { ReactNode } from "react";
import { PrimaryButton } from "./component/button";

type DefaultScreenProps = {
    requestMediaStream: () => void
};

export default function DefaultScreen(
    { requestMediaStream }: Readonly<DefaultScreenProps>
): ReactNode {
    return (
        <div>
            <PrimaryButton onClick={requestMediaStream}>
                Start
            </PrimaryButton>
        </div>
    )
}
