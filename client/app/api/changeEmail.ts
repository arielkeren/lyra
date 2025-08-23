import { put } from "./methods";
import { isAuthResponse } from "./types";

const changeEmail = async (email: string) => {
  const token = localStorage.getItem("token");
  if (!token) return false;

  const data = await put("users", token, {
    email,
    username: "",
    password: "",
  });

  if (!isAuthResponse(data)) return false;

  localStorage.setItem("token", data.token);
  return true;
};

export default changeEmail;
