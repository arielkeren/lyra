import { post } from "./methods";
import { isAuthResponse } from "./types";

const register = async (username: string, email: string, password: string) => {
  const data = await post("auth/register", {
    username,
    email,
    password,
  });
  if (!isAuthResponse(data)) return false;

  localStorage.setItem("token", data.token);
  return true;
};

export default register;
