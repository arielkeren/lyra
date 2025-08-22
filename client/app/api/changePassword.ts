import { put } from "./methods";

const changePassword = async (password: string, token: string) =>
  await put("users", token, {
    password,
    username: "",
    email: "",
  });

export default changePassword;
