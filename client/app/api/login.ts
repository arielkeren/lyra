import { post } from "./methods";
import { isAuthResponse } from "./types";

const login = async (email: string, password: string) => {
  const data = await post("auth/login", {
    email,
    password,
  });
  return isAuthResponse(data) ? data.token : null;
};

export default login;
