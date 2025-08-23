import { put } from "./methods";
import { isAuthResponse } from "./types";

const changePassword = async (password: string) => {
  const token = localStorage.getItem("token");
  if (!token) return false;

  const data = await put("users", token, {
    password,
    username: "",
    email: "",
  });

  if (!isAuthResponse(data)) return false;

  localStorage.setItem("token", data.token);
  return true;
};

export default changePassword;
