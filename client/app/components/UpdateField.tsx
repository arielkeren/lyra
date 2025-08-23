type Props = {
  label: string;
  type: "text" | "email" | "password";
  placeholder: string;
  ref: React.Ref<HTMLInputElement | null>;
  onClick: () => void;
  isEnabled: boolean;
  error: string;
};

const UpdateField = ({
  label,
  type,
  placeholder,
  ref,
  onClick,
  isEnabled,
  error,
}: Props) => {
  return (
    <div className="space-y-2">
      <label className="text-white font-medium">{label}</label>
      <div className="flex gap-2">
        <input
          type={type}
          placeholder={placeholder}
          className="flex-1 px-4 py-2 rounded-lg bg-indigo-950 text-white border border-fuchsia-400 focus:outline-none focus:border-fuchsia-300"
          ref={ref}
        />

        <button
          onClick={onClick}
          className="p-2.5 cursor-pointer bg-gradient-to-r hover:from-fuchsia-500 hover:via-pink-500 hover:to-purple-500 text-white font-semibold rounded-xl shadow-lg border-2 border-fuchsia-300/40 hover:border-fuchsia-200/60 transition-all duration-200 hover:scale-105 hover:shadow-xl"
          disabled={!isEnabled}
        >
          Update
        </button>
      </div>
      {error && <div className="text-red-400 text-sm mt-2">{error}</div>}
    </div>
  );
};

export default UpdateField;
