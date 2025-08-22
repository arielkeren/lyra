import Modal from "./Modal";
import { useState } from "react";
import useUser from "../hooks/useUser";

type Props = {
  isOpen: boolean;
  onClose: () => void;
};

const SettingsModal = ({ isOpen, onClose }: Props) => {
  const { user } = useUser();
  const [username, setUsername] = useState(user?.username || "");
  const [email, setEmail] = useState(user?.email || "");
  const [password, setPassword] = useState("");
  const [loadingField, setLoadingField] = useState<string | null>(null);
  const [errors, setErrors] = useState<Record<string, string>>({});
  const [success, setSuccess] = useState<Record<string, boolean>>({});

  const updateField = async (field: string, value: string) => {
    setLoadingField(field);
    setErrors(prev => ({ ...prev, [field]: "" }));
    setSuccess(prev => ({ ...prev, [field]: false }));

    try {
      // Here you would call your API to update the specific field
      // const result = await updateUser(field, value, localStorage.getItem("token"));

      // Simulate API call
      await new Promise(resolve => setTimeout(resolve, 1000));

      // Show success state
      setSuccess(prev => ({ ...prev, [field]: true }));
      setTimeout(() => setSuccess(prev => ({ ...prev, [field]: false })), 2000);
    } catch (error) {
      setErrors(prev => ({
        ...prev,
        [field]: "Update failed. Please try again.",
      }));
    } finally {
      setLoadingField(null);
    }
  };

  const handleUpdateUsername = () => updateField("username", username);
  const handleUpdateEmail = () => updateField("email", email);
  const handleUpdatePassword = () => updateField("password", password);

  if (!user) return null;

  return (
    <Modal isOpen={isOpen} onClose={onClose} title="Settings">
      <div className="space-y-4">
        <div className="space-y-2">
          <label className="text-white font-medium">Username</label>
          <div className="flex gap-2">
            <input
              type="text"
              placeholder={user.username}
              className="flex-1 px-4 py-2 rounded-lg bg-indigo-950 text-white border border-fuchsia-400 focus:outline-none focus:border-fuchsia-300"
              value={username}
              onChange={e => setUsername(e.target.value)}
            />
            <button
              onClick={handleUpdateUsername}
              className="p-2.5 cursor-pointer bg-gradient-to-r hover:from-fuchsia-500 hover:via-pink-500 hover:to-purple-500 text-white font-semibold rounded-xl shadow-lg border-2 border-fuchsia-300/40 hover:border-fuchsia-200/60 transition-all duration-200 hover:scale-105 hover:shadow-xl"
            >
              Update
            </button>
          </div>
          {errors.username && (
            <div className="text-red-400 text-sm">{errors.username}</div>
          )}
        </div>
      </div>
    </Modal>
  );
};

export default SettingsModal;
