import { post } from "./methods";
import { isAuthResponse } from "./types";

const login = async (email: string, password: string) => {
  const data = await post("auth/login", {
    email,
    password,
  });
  if (!isAuthResponse(data)) return false;

  localStorage.setItem("token", data.token);
  return true;
};

export default login;
