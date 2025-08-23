import { put } from "./methods";
import { isAuthResponse } from "./types";

const changeUsername = async (username: string) => {
  const token = localStorage.getItem("token");
  if (!token) return false;

  const data = await put("users", token, {
    username,
    email: "",
    password: "",
  });

  if (!isAuthResponse(data)) return false;

  localStorage.setItem("token", data.token);
  return true;
};

export default changeUsername;
