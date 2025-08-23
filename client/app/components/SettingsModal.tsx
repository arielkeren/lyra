import Modal from "./Modal";
import { useRef, useState } from "react";
import useUser from "../hooks/useUser";
import changeUsername from "../api/changeUsername";
import changeEmail from "../api/changeEmail";
import changePassword from "../api/changePassword";
import UpdateField from "./UpdateField";

type Props = {
  isOpen: boolean;
  onClose: () => void;
};

const SettingsModal = ({ isOpen, onClose }: Props) => {
  const { user, refreshUser, logout } = useUser();
  const usernameRef = useRef<HTMLInputElement | null>(null);
  const emailRef = useRef<HTMLInputElement | null>(null);
  const passwordRef = useRef<HTMLInputElement | null>(null);

  const [usernameLoading, setUsernameLoading] = useState(false);
  const [emailLoading, setEmailLoading] = useState(false);
  const [passwordLoading, setPasswordLoading] = useState(false);

  const [usernameError, setUsernameError] = useState("");
  const [emailError, setEmailError] = useState("");
  const [passwordError, setPasswordError] = useState("");

  const validateUsername = (username: string): string | null => {
    if (username === "") return "Cannot be empty";
    if (username === user?.username)
      return "Cannot be the same as current username";
    if (username.length < 3) return "Must be at least 3 characters";
    if (username.length > 20) return "Must be at most 20 characters";

    const validUsernameRegex = /^[a-z0-9]+$/;
    if (!validUsernameRegex.test(username))
      return "Only lowercase letters and numbers allowed";
    if (!/^[a-z]/.test(username)) return "Must start with a letter";

    return null;
  };

  const validateEmail = (email: string): string | null => {
    if (email === "") return "Cannot be empty";
    if (email === user?.email) return "Cannot be the same as current email";
    if (email.length > 254) return "Email must be at most 254 characters";

    const tempInput = document.createElement("input");
    tempInput.type = "email";
    tempInput.value = email;

    if (!tempInput.validity.valid) return "Invalid email format";

    return null;
  };

  const validatePassword = (password: string): string | null => {
    if (password === "") return "Cannot be empty";
    if (password.length < 8) return "Must be at least 8 characters";
    if (password.length > 32) return "Must be at most 32 characters";
    if (!/(?=.*[a-zA-Z])(?=.*\d)/.test(password))
      return "Must contain at least one letter and one number";

    return null;
  };

  const updateUsername = async () => {
    if (!usernameRef.current) return;

    const newUsername = usernameRef.current.value.trim();

    setUsernameError("");

    const validationError = validateUsername(newUsername);
    if (validationError) return setUsernameError(validationError);

    setUsernameLoading(true);

    if (await changeUsername(newUsername)) refreshUser();
    else setUsernameError("Failed to update username");

    setUsernameLoading(false);
    usernameRef.current.value = "";
  };

  const updateEmail = async () => {
    if (!emailRef.current) return;

    const newEmail = emailRef.current.value.trim();

    setEmailError("");

    const validationError = validateEmail(newEmail);
    if (validationError) return setEmailError(validationError);

    setEmailLoading(true);

    if (await changeEmail(newEmail)) refreshUser();
    else setEmailError("Failed to update email");

    setEmailLoading(false);
    emailRef.current.value = "";
  };

  const updatePassword = async () => {
    if (!passwordRef.current) return;

    const newPassword = passwordRef.current.value;

    setPasswordError("");

    const validationError = validatePassword(newPassword);
    if (validationError) return setPasswordError(validationError);

    setPasswordLoading(true);

    if (!(await changePassword(newPassword)))
      setPasswordError("Failed to update password");

    setPasswordLoading(false);
    passwordRef.current.value = "";
  };

  const handleLogout = () => {
    logout();
    onClose(); // Close the modal after logout
  };

  if (!user) return null;

  return (
    <Modal isOpen={isOpen} onClose={onClose} title="Settings">
      <div className="space-y-6">
        <UpdateField
          label="Username"
          type="text"
          placeholder={user.username}
          ref={usernameRef}
          onClick={updateUsername}
          isEnabled={!usernameLoading}
          error={usernameError}
        />

        <UpdateField
          label="Email"
          type="email"
          placeholder={user.email}
          ref={emailRef}
          onClick={updateEmail}
          isEnabled={!emailLoading}
          error={emailError}
        />

        <UpdateField
          label="Password"
          type="password"
          placeholder="Enter new password"
          ref={passwordRef}
          onClick={updatePassword}
          isEnabled={!passwordLoading}
          error={passwordError}
        />

        <div className="pt-4 border-t border-white/20">
          <button
            onClick={handleLogout}
            className="py-2.5 w-full cursor-pointer bg-gradient-to-r hover:from-fuchsia-500 hover:via-pink-500 hover:to-purple-500 text-white font-semibold rounded-xl shadow-lg border-2 border-fuchsia-300/40 hover:border-fuchsia-200/60 transition-all duration-200 hover:scale-105 hover:shadow-xl"
          >
            Log Out
          </button>
        </div>
      </div>
    </Modal>
  );
};

export default SettingsModal;
