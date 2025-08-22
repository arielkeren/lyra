type Props = {
  isOpen: boolean;
  onClose: () => void;
  title: string;
  children: React.ReactNode;
};

const Modal = ({ isOpen, onClose, title, children }: Props) =>
  !isOpen ? null : (
    <div className="fixed inset-0 z-50 flex items-center justify-center backdrop-blur-sm">
      <div className="bg-gradient-to-br from-indigo-900 via-purple-900 to-fuchsia-900 text-white rounded-2xl shadow-2xl p-8 w-full max-w-md relative">
        <button
          onClick={onClose}
          className="cursor-pointer absolute top-4 right-4 text-fuchsia-300 hover:text-fuchsia-100 text-2xl font-bold"
          aria-label="Close"
        >
          &times;
        </button>
        <h2 className="text-3xl font-bold bg-gradient-to-r from-fuchsia-300 to-purple-300 bg-clip-text text-transparent text-center">
          {title}
        </h2>
        {children}
      </div>
    </div>
  );

export default Modal;
