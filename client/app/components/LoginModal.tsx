import login from "../api/login";
import useUser from "../hooks/useUser";
import Modal from "./Modal";
import { useState } from "react";

type Props = {
  isOpen: boolean;
  onClose: () => void;
};

const LoginModal = ({ isOpen, onClose }: Props) => {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [isError, setIsError] = useState(false);
  const { refreshUser } = useUser();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsLoading(true);

    if (await login(email, password)) {
      refreshUser();
      onClose();
    } else setIsError(true);

    setIsLoading(false);
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose} title="Log In">
      <form className="space-y-4" onSubmit={handleSubmit}>
        <input
          type="email"
          placeholder="Email"
          className="w-full px-4 py-2 rounded-lg bg-indigo-950 text-white border border-fuchsia-400 focus:outline-none"
          value={email}
          onChange={e => setEmail(e.target.value)}
          required
        />
        <input
          type="password"
          placeholder="Password"
          className="w-full px-4 py-2 rounded-lg bg-indigo-950 text-white border border-fuchsia-400 focus:outline-none"
          value={password}
          onChange={e => setPassword(e.target.value)}
          required
        />
        {isError && (
          <div className="text-red-400 text-sm text-center">Login failed</div>
        )}
        <button
          type="submit"
          className="w-full bg-gradient-to-r from-fuchsia-500 via-pink-500 to-purple-500 text-white font-bold py-2 rounded-lg shadow-lg border-2 border-fuchsia-300 mt-2 cursor-pointer"
          disabled={isLoading}
        >
          {isLoading ? "Logging in..." : "Log in"}
        </button>
      </form>
    </Modal>
  );
};

export default LoginModal;
