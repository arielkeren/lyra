import { post } from "./methods";
import { isAuthResponse } from "./types";

const register = async (username: string, email: string, password: string) => {
  const data = await post("auth/register", {
    username,
    email,
    password,
  });
  return isAuthResponse(data) ? data.token : null;
};

export default register;
