interface BoxProps {
    children: React.ReactNode;
    height?: string | number;
    width?: string | number;
}

export function Box({ children, height = "auto", width = "auto" }: BoxProps) {
    return (
        <div className="bg-pink-300 p-16 rounded-3xl shadow-xl" style={{ height, width }}>
            {children}
        </div>
    );
}

export function MainText({ text }: { text: string }) {
    return <h1 className="text-3xl font-bold text-purple-700">{text}</h1>;
}

export function Description({ children }: { children: React.ReactNode }) {
    return <p className="text-lg text-blue-600">{children}</p>;
}

export function FancyInput({ label, placeholder, type, onChange }: { label: string, placeholder: string, type: string, onChange: (e: React.ChangeEvent<HTMLInputElement>) => void }) {
    return (
        <div className="mt-5 flex flex-col items-center gap-2">
            {/* <label htmlFor={label} className="text-lg text-gray-700">{label}</label> */}
            <input type={type} id={label} placeholder={placeholder} className="p-2 border-2 w-max text-purple-700 rounded-2xl" onChange={onChange} />
        </div>
    );
}

export function Button({ name, type="button", onClick }: { name: string, type?: "button" | "submit" | "reset", onClick: () => void }) {
    return (
        <button type={type} className="bg-blue-500 text-white font-bold py-2 px-4 rounded-xl hover:bg-blue-700 hover:transform hover:-translate-y-1 transition-all duration-300" onClick={onClick}>
            {name}
        </button>
    );
}