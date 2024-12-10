import type { ReactNode } from "react";

type ButtonProps = {
	children: ReactNode;
	onClick: () => void;
};

export function PrimaryButton({
	children,
	onClick,
}: Readonly<ButtonProps>): ReactNode {
	return (
		<button
			type="button"
			className="bg-blue-700 text-slate-100 px-4 py-2 rounded-lg hover:bg-blue-500 active:bg-blue-900"
			onClick={onClick}
		>
			{children}
		</button>
	);
}
